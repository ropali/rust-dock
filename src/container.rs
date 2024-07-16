use std;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
//Labels, HostConfig
pub struct Container {
    pub Id: String,
    pub Image: String,
    pub Status: String,
    pub Command: String,
    pub Created: u64,
    pub Names: Vec<String>,
    pub Ports: Vec<Port>,
    pub SizeRw: Option<u64>, // I guess it is optional on Mac.
    pub SizeRootFs: u64,
    pub Labels: Option<HashMap<String, String>>,
    pub HostConfig: HostConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Port {
    pub IP: Option<String>,
    pub PrivatePort: u64,
    pub PublicPort: Option<u64>,
    pub Type: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LogConfig {
    pub Type: String,
    pub Config: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HostConfig {
    pub NetworkMode: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct State {
    pub Status: String,
    pub Running: bool,
    pub Paused: bool,
    pub Restarting: bool,
    pub OOMKilled: bool,
    pub Dead: bool,
    pub Pid: i64,
    pub ExitCode: i8,
    pub Error: String,
    pub StartedAt: String,
    pub FinishedAt: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Config {
    Hostname: String,
    Domainname: String,
    User: String,
    AttachStdin: bool,
    AttachStdout: bool,
    AttachStderr: bool,
    Tty: bool,
    OpenStdin: bool,
    StdinOnce: bool,
    Env: Vec<String>,
    Cmd: Vec<String>,
    Image: String,
    Volumes: Option<Vec<String>>,
    WorkingDir: String,
    Entrypoint: Option<Vec<String>>,
    OnBuild: Option<Vec<String>>,
    Labels: serde_json::Value, // Using serde_json::Value for flexibility
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ContainerInfo {
    pub AppArmorProfile: String,
    pub Args: Vec<String>,
    pub Config: Config,
    pub Created: String,
    pub Driver: String,

    pub HostConfig: HostConfig,
    pub HostnamePath: String,
    pub HostsPath: String,
    pub LogPath: String,
    pub Id: String,
    pub Image: String,
    pub MountLabel: String,
    pub Name: String,
    // NetworkSettings
    pub Path: String,
    pub ProcessLabel: String,
    pub ResolvConfPath: String,
    pub RestartCount: u64,
    pub Platform: String,
    pub ExecIDs: Option<Vec<String>>,
    pub Mounts: Vec<String>,

    pub State: State
}

impl Clone for Container {
    fn clone(&self) -> Self {
        let container = Container {
            Id: self.Id.clone(),
            Image: self.Image.clone(),
            Status: self.Status.clone(),
            Command: self.Command.clone(),
            Created: self.Created.clone(),
            Names: self.Names.clone(),
            Ports: self.Ports.clone(),
            SizeRw: self.SizeRw,
            SizeRootFs: self.SizeRootFs,
            Labels: self.Labels.clone(),
            HostConfig: self.HostConfig.clone(),
        };

        return container;
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Id)
    }
}

impl std::clone::Clone for Port {
    fn clone(&self) -> Self {
        let port = Port {
            IP: self.IP.clone(),
            PrivatePort: self.PrivatePort.clone(),
            PublicPort: self.PublicPort.clone(),
            Type: self.Type.clone(),
        };
        return port;
    }
}

impl Clone for HostConfig {
    fn clone(&self) -> Self {
        let host_config = HostConfig {
            NetworkMode: self.NetworkMode.clone()
        };
        return host_config;
    }
}

impl Clone for ContainerInfo {
    fn clone(&self) -> Self {
        let container_info = ContainerInfo {
            AppArmorProfile: self.AppArmorProfile.clone(),
            Args: self.Args.clone(),
            // Config
            Config: self.Config.clone(),
            Created: self.Created.clone(),
            Driver: self.Driver.clone(),
            // ExecDriver: self.ExecDriver.clone(),
            // ExecIDs
            // HostConfig
            HostConfig: self.HostConfig.clone(),
            HostnamePath: self.HostnamePath.clone(),
            HostsPath: self.HostsPath.clone(),
            LogPath: self.LogPath.clone(),
            Id: self.Id.clone(),
            Image: self.Image.clone(),
            MountLabel: self.MountLabel.clone(),
            Name: self.Name.clone(),
            // NetworkSettings
            Path: self.Path.clone(),
            ProcessLabel: self.ProcessLabel.clone(),
            ResolvConfPath: self.ResolvConfPath.clone(),
            RestartCount: self.RestartCount,

            Platform: self.Platform.clone(),
            ExecIDs: self.ExecIDs.clone(),
            Mounts: self.Mounts.clone(),
            State: self.State.clone(),
        };
        return container_info;
    }
}

impl std::fmt::Display for ContainerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PortBinding {
    pub HostIp: Option<String>,
    pub HostPort: String,
}

impl Clone for PortBinding {
    fn clone(&self) -> Self {
        PortBinding {
            HostIp: self.HostIp.clone(),
            HostPort: self.HostPort.clone(),
        }
    }
}

impl std::fmt::Display for PortBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.HostPort)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HostConfigCreate {
    pub NetworkMode: Option<String>,
    pub PublishAllPorts: Option<bool>,
    pub PortBindings: Option<HashMap<String, Vec<PortBinding>>>,
}

impl Clone for HostConfigCreate {
    fn clone(&self) -> Self {
        HostConfigCreate {
            NetworkMode: self.NetworkMode.clone(),
            PublishAllPorts: self.PublishAllPorts.clone(),
            PortBindings: self.PortBindings.clone(),
        }
    }
}

impl std::fmt::Display for HostConfigCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self.NetworkMode)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ContainerCreate {
    pub Image: String,
    pub Labels: Option<HashMap<String, String>>,
    pub ExposedPorts: Option<HashMap<String, HashMap<i32, i32>>>,
    pub HostConfig: Option<HostConfigCreate>,
}

impl Clone for ContainerCreate {
    fn clone(&self) -> Self {
        ContainerCreate {
            Image: self.Image.clone(),
            Labels: self.Labels.clone(),
            ExposedPorts: self.ExposedPorts.clone(),
            HostConfig: self.HostConfig.clone(),
        }
    }
}

impl std::fmt::Display for ContainerCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Image)
    }
}
