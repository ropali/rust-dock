use std;
use std::error::Error as StdError;
use std::io::Error;

use hyper::{Body, Client, Method, Request, Uri};
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyperlocal::{UnixConnector, Uri as HyperlocalUri};
use serde::de::DeserializeOwned;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;

use crate::container::{Container, ContainerCreate, ContainerInfo};
use crate::filesystem::FilesystemChange;
use crate::futures::FutureExt;
use crate::futures::TryFutureExt;
use crate::image::{Image, ImageHistory, ImageStatus};
use crate::network::{Network, NetworkCreate};
use crate::process::{Process, Top};
use crate::stats::Stats;
use crate::system::SystemInfo;
use crate::version::Version;
use crate::volume::{Volume, VolumesResponse};

#[derive(Clone)]
pub struct Docker {
    protocol: Protocol,
    path: String,
    hyperlocal_client: Option<Client<UnixConnector, Body>>,
    hyper_client: Option<Client<HttpConnector, Body>>,
}

#[derive(Clone)]
enum Protocol {
    UNIX,
    TCP,
}

impl Docker {
    pub fn connect(addr: &str) -> std::io::Result<Docker> {
        let components: Vec<&str> = addr.split("://").collect();
        if components.len() != 2 {
            let err =
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "The address is invalid.");
            return Err(err);
        }

        let protocol = components[0];
        let path = components[1].to_string();

        let protocol = match protocol {
            "unix" => Protocol::UNIX,
            "tcp" => Protocol::TCP,
            _ => {
                let err = std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The protocol is not supported.",
                );
                return Err(err);
            }
        };

        let hyperlocal_client = match protocol {
            Protocol::UNIX => {
                let unix_connector = UnixConnector;
                Some(Client::builder().build(unix_connector))
            }
            _ => None,
        };

        let hyper_client = match protocol {
            Protocol::TCP => Some(Client::new()),
            _ => None,
        };

        let docker = Docker {
            protocol: protocol,
            path: path,
            hyperlocal_client: hyperlocal_client,
            hyper_client: hyper_client,
        };
        return Ok(docker);
    }

    fn request_file(&self, method: Method, url: &str, file: Vec<u8>, content_type: &str) -> String {
        let req = Request::builder()
            .uri(match self.protocol {
                Protocol::UNIX => hyperlocal::Uri::new(self.path.clone(), url).into(),
                _ => format!("{}{}", self.path, url).parse::<Uri>().unwrap(),
            })
            .header("Content-Type", content_type)
            .header("Accept", "application/json")
            .method(method)
            .body(Body::from(file))
            .expect("failed to build request");
        match Handle::try_current() {
            Ok(handle) => handle.block_on(
                match self.protocol {
                    Protocol::UNIX => self.hyperlocal_client.as_ref().unwrap().request(req),
                    Protocol::TCP => self.hyper_client.as_ref().unwrap().request(req),
                }
                    .and_then(|res| hyper::body::to_bytes(res.into_body()))
                    .map(|body| {
                        String::from_utf8(body.expect("Body should not have an error").to_vec())
                            .unwrap()
                    }),
            ),
            Err(_) => Runtime::new().unwrap().block_on(
                match self.protocol {
                    Protocol::UNIX => self.hyperlocal_client.as_ref().unwrap().request(req),
                    Protocol::TCP => self.hyper_client.as_ref().unwrap().request(req),
                }
                    .and_then(|res| hyper::body::to_bytes(res.into_body()))
                    .map(|body| {
                        String::from_utf8(body.expect("Body should not have an error").to_vec())
                            .unwrap()
                    }),
            ),
        }
    }

    fn request(&self, method: Method, url: &str, body: String) -> String {
        self.request_file(method, url, body.into_bytes(), "application/json")
    }


    //
    // Networks
    //

    pub fn get_networks(&mut self) -> std::io::Result<Vec<Network>> {
        let body = self.request(Method::GET, "/networks", "".to_string());

        match serde_json::from_str(&body) {
            Ok(networks) => Ok(networks),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn create_network(&mut self, network: NetworkCreate) -> std::io::Result<String> {
        let body = self.request(
            Method::POST,
            "/networks/create",
            serde_json::to_string(&network).unwrap(),
        );

        let status: serde_json::Value = match serde_json::from_str(&body) {
            Ok(status) => status,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    e.to_string(),
                ));
            }
        };
        match status.get("Id") {
            Some(id) => Ok(id.as_str().unwrap().to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status.get("message").unwrap().to_string(),
            )),
        }
    }

    pub fn delete_network(&mut self, id_or_name: &str) -> std::io::Result<String> {
        let body = self.request(
            Method::DELETE,
            &format!("/networks/{}", id_or_name),
            "".to_string(),
        );

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(status) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status["message"].to_string(),
            )),
            Err(_e) => Ok("".to_string()),
        }
    }

    //
    // Containers
    //

    pub async fn stream_container_logs(&self, container_id: &str, sender: mpsc::Sender<String>) -> Result<(), Box<dyn StdError>> {
        let url = format!("/containers/{}/logs?stdout=true&stderr=true&follow=true", container_id);
        let uri = match self.protocol {
            Protocol::UNIX => HyperlocalUri::new(&self.path, &url).into(),
            Protocol::TCP => format!("{}{}", self.path, url).parse::<Uri>().unwrap(),
        };

        let req = hyper::Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("Accept", "application/json")
            .body(hyper::Body::empty())
            .expect("request builder");

        let res = match self.protocol {
            Protocol::UNIX => self.hyperlocal_client.as_ref().unwrap().request(req).await?,
            Protocol::TCP => self.hyper_client.as_ref().unwrap().request(req).await?,
        };

        let mut body = res.into_body();

        while let Some(chunk) = body.data().await {
            let chunk = chunk?;
            let chunk_str = String::from_utf8_lossy(&chunk).to_string();
            sender.send(chunk_str).await.expect("Failed to send log chunk");
        }

        Ok(())
    }

    pub fn start_container(&mut self, id_or_name: &str) -> std::io::Result<String> {
        let body = self.request(
            Method::POST,
            &format!("/containers/{}/start", id_or_name),
            "".to_string(),
        );

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(status) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status["message"].to_string(),
            )),
            Err(_e) => Ok("".to_string()),
        }
    }

    pub fn stop_container(&mut self, id_or_name: &str) -> std::io::Result<String> {
        let body = self.request(
            Method::POST,
            &format!("/containers/{}/stop", id_or_name),
            "".to_string(),
        );

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(status) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status["message"].to_string(),
            )),
            Err(_e) => Ok("".to_string()),
        }
    }

    pub fn delete_container(&mut self, id_or_name: &str) -> std::io::Result<String> {
        let body = self.request(
            Method::DELETE,
            &format!("/containers/{}", id_or_name),
            "".to_string(),
        );

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(status) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status["message"].to_string(),
            )),
            Err(_e) => Ok("".to_string()),
        }
    }

    pub fn create_container(
        &mut self,
        name: String,
        container: ContainerCreate,
    ) -> std::io::Result<String> {
        let body = self.request(
            Method::POST,
            &format!("/containers/create?name={}", name),
            serde_json::to_string(&container).unwrap(),
        );

        let status: serde_json::Value = match serde_json::from_str(&body) {
            Ok(status) => status,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    e.to_string(),
                ));
            }
        };
        match status.get("Id") {
            Some(id) => Ok(id.as_str().unwrap().to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status.get("message").unwrap().to_string(),
            )),
        }
    }

    pub fn get_containers(&mut self, all: bool) -> std::io::Result<Vec<Container>> {
        let a = match all {
            true => "1",
            false => "0",
        };

        let body = self.request(
            Method::GET,
            &format!("/containers/json?all={}&size=1", a),
            "".to_string(),
        );

        let containers: Vec<Container> = match serde_json::from_str(&body) {
            Ok(containers) => containers,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };

        return Ok(containers);
    }


    pub fn get_container_info(&mut self, id_or_name: &str) -> std::io::Result<ContainerInfo> {
        self.get_container_info_generic(id_or_name)
    }

    pub fn get_container_info_raw(
        &mut self,
        id_or_name: &str,
    ) -> std::io::Result<serde_json::Value> {
        self.get_container_info_generic(id_or_name)
    }

    fn get_container_info_generic<T: DeserializeOwned>(
        &mut self,
        id_or_name: &str,
    ) -> std::io::Result<T> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/json", id_or_name),
            "".to_string(),
        );

        match serde_json::from_str(&body) {
            Ok(parsed_body) => Ok(parsed_body),
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                Err(err)
            }
        }
    }

    pub fn get_processes(&mut self, container: &Container) -> std::io::Result<Vec<Process>> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/top", container.Id),
            "".to_string(),
        );

        let top: Top = match serde_json::from_str(&body) {
            Ok(top) => top,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };

        let mut processes: Vec<Process> = Vec::new();
        let mut process_iter = top.Processes.iter();
        loop {
            let process = match process_iter.next() {
                Some(process) => process,
                None => {
                    break;
                }
            };

            let mut p = Process {
                user: String::new(),
                pid: String::new(),
                cpu: None,
                memory: None,
                vsz: None,
                rss: None,
                tty: None,
                stat: None,
                start: None,
                time: None,
                command: String::new(),
            };

            let mut value_iter = process.iter();
            let mut i: usize = 0;
            loop {
                let value = match value_iter.next() {
                    Some(value) => value,
                    None => {
                        break;
                    }
                };
                let key = &top.Titles[i];
                match key.as_ref() {
                    "USER" => p.user = value.clone(),
                    "PID" => p.pid = value.clone(),
                    "%CPU" => p.cpu = Some(value.clone()),
                    "%MEM" => p.memory = Some(value.clone()),
                    "VSZ" => p.vsz = Some(value.clone()),
                    "RSS" => p.rss = Some(value.clone()),
                    "TTY" => p.tty = Some(value.clone()),
                    "STAT" => p.stat = Some(value.clone()),
                    "START" => p.start = Some(value.clone()),
                    "TIME" => p.time = Some(value.clone()),
                    "COMMAND" => p.command = value.clone(),
                    _ => {}
                }

                i = i + 1;
            }

            processes.push(p);
        }

        return Ok(processes);
    }

    pub fn get_stats(&mut self, container: &Container) -> std::io::Result<Stats> {
        if container.Status.contains("Up") == false {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The container is already stopped.",
            );
            return Err(err);
        }

        let body = self.request(
            Method::GET,
            &format!("/containers/{}/stats", container.Id),
            "".to_string(),
        );

        let stats: Stats = match serde_json::from_str(&body) {
            Ok(stats) => stats,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(stats);
    }

    //
    // Image
    //

    pub fn delete_image(&mut self, id_or_name: &str, force: bool, no_prune: bool) -> std::io::Result<bool> {
        let qs = format!("force={}&noprune={}", force, no_prune);

        let body = self.request(
            Method::DELETE,
            &format!("/images/{}?{}", id_or_name, qs),
            "".to_string(),
        );

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(data) => {
                if data.is_array() {
                    Ok(true)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        data["message"].to_string(),
                    ))
                }
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn build_image(&mut self, data: Vec<u8>, t: String) -> std::io::Result<String> {
        let body = self.request_file(
            Method::POST,
            &format!("/build?t={}", t),
            data,
            "application/x-tar",
        );
        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(status) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status["message"].to_string(),
            )),
            Err(_e) => Ok("".to_string()),
        }
    }

    pub fn create_image(
        &mut self,
        image: String,
        tag: String,
    ) -> std::io::Result<Vec<ImageStatus>> {
        let body = format!(
            "[{}]",
            self.request(
                Method::POST,
                &format!("/images/create?fromImage={}&tag={}", image, tag),
                "".to_string(),
            )
        );
        let fixed = body.replace("}{", "},{");

        let statuses: Vec<ImageStatus> = match serde_json::from_str(&fixed) {
            Ok(statuses) => statuses,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(statuses);
    }

    pub fn get_images(&mut self, all: bool) -> std::io::Result<Vec<Image>> {
        let a = match all {
            true => "1",
            false => "0",
        };
        let body = self.request(
            Method::GET,
            &format!("/images/json?all={}", a),
            "".to_string(),
        );

        let images: Vec<Image> = match serde_json::from_str(&body) {
            Ok(images) => images,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(images);
    }


    pub fn inspect_image(&mut self, name: &str) -> std::io::Result<serde_json::Value> {
        let body = self.request(Method::GET, &format!("/images/{}/json", name), "".to_string());

        let images: serde_json::Value = match serde_json::from_str(&body) {
            Ok(images) => images,
            Err(e) => {
                let err = Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(images);
    }


    pub fn image_history(&mut self, name: &str) -> std::io::Result<Vec<ImageHistory>> {
        let body = self.request(Method::GET, &format!("/images/{}/history", name), "".to_string());

        let images: Vec<ImageHistory> = match serde_json::from_str(&body) {
            Ok(images) => images,
            Err(e) => {
                let err = Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(images);
    }

    pub fn get_system_info(&mut self) -> std::io::Result<SystemInfo> {
        let body = self.request(Method::GET, "/info", "".to_string());

        let info: SystemInfo = match serde_json::from_str(&body) {
            Ok(info) => info,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(info);
    }


    pub fn get_filesystem_changes(
        &mut self,
        container: &Container,
    ) -> std::io::Result<Vec<FilesystemChange>> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/changes", container.Id),
            "".to_string(),
        );

        let filesystem_changes: Vec<FilesystemChange> = match serde_json::from_str(&body) {
            Ok(body) => body,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(filesystem_changes);
    }

    pub fn export_container(&mut self, container: &Container) -> std::io::Result<String> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/export", container.Id),
            "".to_string(),
        );

        return Ok(body);
    }


    //
    // Volume
    //

    pub fn get_volumes(&mut self) -> std::io::Result<Vec<Volume>> {
        let body = self.request(
            Method::GET,
            &"/volumes".to_string(),
            "".to_string(),
        );

        let data: VolumesResponse = match serde_json::from_str(&body) {
            Ok(body) => body,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(data.Volumes);
    }


    pub fn inspect_volume(&mut self, id_or_name: &str) -> std::io::Result<Volume> {
        let body = self.request(
            Method::GET,
            &format!("/volumes/{}", id_or_name),
            "".to_string(),
        );

        let data: Volume = match serde_json::from_str(&body) {
            Ok(body) => body,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(data);
    }



    pub fn ping(&mut self) -> std::io::Result<String> {
        let body = self.request(Method::GET, "/_ping", "".to_string());

        return Ok(body);
    }

    pub fn get_version(&mut self) -> std::io::Result<Version> {
        let body = self.request(Method::GET, "/version", "".to_string());

        let version: Version = match serde_json::from_str(&body) {
            Ok(r_body) => r_body,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };
        return Ok(version);
    }
}
