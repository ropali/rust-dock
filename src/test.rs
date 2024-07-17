#[cfg(test)]
use crate::container::{Container, ContainerInfo};
#[cfg(test)]
use crate::filesystem::FilesystemChange;
#[cfg(test)]
use crate::image::Image;
#[cfg(test)]
use crate::network::Network;
#[cfg(test)]
use crate::process::Top;
#[cfg(test)]
use crate::stats::Stats;
#[cfg(test)]
use crate::system::SystemInfo;
#[cfg(test)]
use crate::version::Version;

#[test]
#[cfg(test)]
fn get_networks() {
    let response = get_networks_response();
    let _: Vec<Network> = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_containers() {
    let response = get_containers_response();
    let _: Vec<Container> = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_stats() {
    let response = get_stats_response();
    let _: Stats = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_system_info() {
    let response = get_system_info_response();
    let _: SystemInfo = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_images() {
    let response = get_images_response();
    let _: Vec<Image> = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_container_info() {
    let response = get_container_info_response();
    let _: ContainerInfo = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_container_info_raw() {
    let response = get_container_info_response();
    let _: serde_json::Value = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_processes() {
    let response = get_processes_response();
    let _: Top = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_filesystem_changes() {
    let response = get_filesystem_changes_response();
    let _: Vec<FilesystemChange> = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[test]
#[cfg(test)]
fn get_version() {
    let response = get_version_response();
    let _: Version = match serde_json::from_str(&response) {
        Ok(body) => body,
        Err(_) => {
            assert!(false);
            return;
        }
    };
}

#[cfg(test)]
fn get_networks_response() -> String {
    return "[{\"Name\":\"bridge\",\"Id\":\"f2de39df4171b0dc801e8002d1d999b77256983dfc63041c0f34030aa3977566\",\"Created\":\"2016-10-19T06:21:00.416543526Z\",\"Scope\":\"local\",\"Driver\":\"bridge\",\"EnableIPv6\":false,\"Internal\":false,\"Attachable\":false,\"Ingress\":false,\"IPAM\":{\"Driver\":\"default\",\"Config\":[{\"Subnet\":\"172.17.0.0/16\"}]},\"Options\":{\"com.docker.network.bridge.default_bridge\":\"true\",\"com.docker.network.bridge.enable_icc\":\"true\",\"com.docker.network.bridge.enable_ip_masquerade\":\"true\",\"com.docker.network.bridge.host_binding_ipv4\":\"0.0.0.0\",\"com.docker.network.bridge.name\":\"docker0\",\"com.docker.network.driver.mtu\":\"1500\"}},{\"Name\":\"none\",\"Id\":\"e086a3893b05ab69242d3c44e49483a3bbbd3a26b46baa8f61ab797c1088d794\",\"Created\":\"0001-01-01T00:00:00Z\",\"Scope\":\"local\",\"Driver\":null,\"EnableIPv6\":false,\"Internal\":false,\"Attachable\":false,\"Ingress\":false,\"IPAM\":{\"Driver\":\"default\",\"Config\":[]},\"Containers\":{},\"Options\":{}},{\"Name\":\"host\",\"Id\":\"13e871235c677f196c4e1ecebb9dc733b9b2d2ab589e30c539efeda84a24215e\",\"Created\":\"0001-01-01T00:00:00Z\",\"Scope\":\"local\",\"Driver\":\"host\",\"EnableIPv6\":false,\"Internal\":false,\"Attachable\":false,\"Ingress\":false,\"IPAM\":{\"Driver\":\"default\",\"Config\":[]},\"Containers\":{},\"Options\":{},\"Labels\":{}}]".to_string();
}

#[cfg(test)]
fn get_containers_response() -> String {
    return "[{\"Id\":\"ed3221f4adc05b9ecfbf56b1aa76d4e6e70d5b73b3876c322fc10d017c64ca86\",\"Names\":[\"/rust\"],\"Image\":\"ghmlee/rust:latest\",\"Command\":\"bash\",\"Created\":1439434052,\"Ports\":[{\"IP\":\"0.0.0.0\",\"PrivatePort\":8888,\"PublicPort\":8888,\"Type\":\"tcp\"}],\"SizeRootFs\":253602755,\"Labels\":{},\"Status\":\"Exited (137) 12 hours ago\",\"HostConfig\":{\"NetworkMode\":\"default\"},\"SizeRw\":10832473}]".to_string();
}

#[cfg(test)]
fn get_stats_response() -> String {
    return "{\"read\":\"2015-04-09T07:02:08.480022082Z\",\"network\":{\"rx_bytes\":5820720,\"rx_packets\":2742,\"rx_errors\":0,\"rx_dropped\":1,\"tx_bytes\":158527,\"tx_packets\":2124,\"tx_errors\":0,\"tx_dropped\":0},\"cpu_stats\":{\"cpu_usage\":{\"total_usage\":19194125000,\"percpu_usage\":[14110113138,3245604417,845722573,992684872],\"usage_in_kernelmode\":1110000000,\"usage_in_usermode\":18160000000},\"system_cpu_usage\":1014488290000000,\"throttling_data\":{\"periods\":0,\"throttled_periods\":0,\"throttled_time\":0}},\"memory_stats\":{\"usage\":208437248,\"max_usage\":318791680,\"stats\":{\"active_anon\":27213824,\"active_file\":129069056,\"cache\":178946048,\"hierarchical_memory_limit\":18446744073709551615,\"hierarchical_memsw_limit\":18446744073709551615,\"inactive_anon\":0,\"inactive_file\":49876992,\"mapped_file\":10809344,\"pgfault\":99588,\"pgmajfault\":819,\"pgpgin\":130731,\"pgpgout\":153466,\"rss\":29331456,\"rss_huge\":6291456,\"swap\":0,\"total_active_anon\":27213824,\"total_active_file\":129069056,\"total_cache\":178946048,\"total_inactive_anon\":0,\"total_inactive_file\":49876992,\"total_mapped_file\":10809344,\"total_pgfault\":99588,\"total_pgmajfault\":819,\"total_pgpgin\":130731,\"total_pgpgout\":153466,\"total_rss\":29331456,\"total_rss_huge\":6291456,\"total_swap\":0,\"total_unevictable\":0,\"total_writeback\":0,\"unevictable\":0,\"writeback\":0},\"failcnt\":0,\"limit\":16854257664},\"blkio_stats\":{\"io_service_bytes_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":150687744},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":150687744},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":150687744}],\"io_serviced_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":484},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":484},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":484}],\"io_queue_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":0}],\"io_service_time_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":2060941295},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":2060941295},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":2060941295}],\"io_wait_time_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":5476872825},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":5476872825},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":5476872825}],\"io_merged_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":79},{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0},{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":79},{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":79}],\"io_time_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"\",\"value\":1814}],\"sectors_recursive\":[{\"major\":8,\"minor\":0,\"op\":\"\",\"value\":294312}]}}".to_string();
}

#[cfg(test)]
fn get_system_info_response() -> String {
    return "{\"Containers\":6,\"Debug\":0,\"DockerRootDir\":\"/var/lib/docker\",\"Driver\":\"btrfs\",\"DriverStatus\":[[\"Build Version\",\"Btrfs v3.17.1\"],[\"Library Version\",\"101\"]],\"ExecutionDriver\":\"native-0.2\",\"ID\":\"WG63:3NIU:TSI2:FV7J:IL2O:YPXA:JR3F:XEKT:JZVR:JA6T:QMYE:B4SB\",\"IPv4Forwarding\":1,\"Images\":190,\"IndexServerAddress\":\"https://index.docker.io/v1/\",\"InitPath\":\"/usr/libexec/docker/dockerinit\",\"InitSha1\":\"30c93967bdc3634b6036e1a76fd547bbe171b264\",\"KernelVersion\":\"3.18.6\",\"Labels\":null,\"MemTotal\":16854257664,\"MemoryLimit\":1,\"NCPU\":4,\"NEventsListener\":0,\"NFd\":68,\"NGoroutines\":95,\"Name\":\"core\",\"OperatingSystem\":\"CoreOS 607.0.0\",\"RegistryConfig\":{\"IndexConfigs\":{\"docker.io\":{\"Mirrors\":null,\"Name\":\"docker.io\",\"Official\":true,\"Secure\":true}},\"InsecureRegistryCIDRs\":[\"127.0.0.0/8\"]},\"SwapLimit\":1}".to_string();
}

#[cfg(test)]
fn get_images_response() -> String {
    return "[{\"Created\":1428533761,\"Id\":\"533da4fa223bfbca0f56f65724bb7a4aae7a1acd6afa2309f370463eaf9c34a4\",\"ParentId\":\"84ac0b87e42afe881d36f03dea817f46893f9443f9fc10b64ec279737384df12\",\"RepoTags\":[\"ghmlee/rust:nightly\"],\"Size\":0,\"VirtualSize\":806688288},{\"Created\":1371157430,\"Id\":\"511136ea3c5a64f264b78b5433614aec563103b4d4702f3ba7d4d2698e22c158\",\"ParentId\":\"\",\"RepoTags\":[],\"Size\":0,\"VirtualSize\":0}]".to_string();
}

#[cfg(test)]
fn get_container_info_response() -> String {
    return "{\"Id\":\"dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118\",\"Created\":\"2024-06-02T14:56:43.3912062Z\",\"Path\":\"/hello\",\"Args\":[],\"State\":{\"Status\":\"exited\",\"Running\":false,\"Paused\":false,\"Restarting\":false,\"OOMKilled\":false,\"Dead\":false,\"Pid\":0,\"ExitCode\":0,\"Error\":\"\",\"StartedAt\":\"2024-07-14T09:15:09.506227996Z\",\"FinishedAt\":\"2024-07-14T09:15:09.637925962Z\"},\"Image\":\"sha256:d2c94e258dcb3c5ac2798d32e1249e42ef01cba4841c2234249495f87264ac5a\",\"ResolvConfPath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/resolv.conf\",\"HostnamePath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/hostname\",\"HostsPath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/hosts\",\"LogPath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118-json.log\",\"Name\":\"/lucid_roentgen\",\"RestartCount\":0,\"Driver\":\"overlay2\",\"Platform\":\"linux\",\"MountLabel\":\"\",\"ProcessLabel\":\"\",\"AppArmorProfile\":\"docker-default\",\"ExecIDs\":null,\"HostConfig\":{\"Binds\":null,\"ContainerIDFile\":\"\",\"LogConfig\":{\"Type\":\"json-file\",\"Config\":{}},\"NetworkMode\":\"bridge\",\"PortBindings\":{},\"RestartPolicy\":{\"Name\":\"no\",\"MaximumRetryCount\":0},\"AutoRemove\":false,\"VolumeDriver\":\"\",\"VolumesFrom\":null,\"ConsoleSize\":[28,90],\"CapAdd\":null,\"CapDrop\":null,\"CgroupnsMode\":\"private\",\"Dns\":[],\"DnsOptions\":[],\"DnsSearch\":[],\"ExtraHosts\":null,\"GroupAdd\":null,\"IpcMode\":\"private\",\"Cgroup\":\"\",\"Links\":null,\"OomScoreAdj\":0,\"PidMode\":\"\",\"Privileged\":false,\"PublishAllPorts\":false,\"ReadonlyRootfs\":false,\"SecurityOpt\":null,\"UTSMode\":\"\",\"UsernsMode\":\"\",\"ShmSize\":67108864,\"Runtime\":\"runc\",\"Isolation\":\"\",\"CpuShares\":0,\"Memory\":0,\"NanoCpus\":0,\"CgroupParent\":\"\",\"BlkioWeight\":0,\"BlkioWeightDevice\":[],\"BlkioDeviceReadBps\":[],\"BlkioDeviceWriteBps\":[],\"BlkioDeviceReadIOps\":[],\"BlkioDeviceWriteIOps\":[],\"CpuPeriod\":0,\"CpuQuota\":0,\"CpuRealtimePeriod\":0,\"CpuRealtimeRuntime\":0,\"CpusetCpus\":\"\",\"CpusetMems\":\"\",\"Devices\":[],\"DeviceCgroupRules\":null,\"DeviceRequests\":null,\"MemoryReservation\":0,\"MemorySwap\":0,\"MemorySwappiness\":null,\"OomKillDisable\":null,\"PidsLimit\":null,\"Ulimits\":[],\"CpuCount\":0,\"CpuPercent\":0,\"IOMaximumIOps\":0,\"IOMaximumBandwidth\":0,\"MaskedPaths\":[\"/proc/asound\",\"/proc/acpi\",\"/proc/kcore\",\"/proc/keys\",\"/proc/latency_stats\",\"/proc/timer_list\",\"/proc/timer_stats\",\"/proc/sched_debug\",\"/proc/scsi\",\"/sys/firmware\",\"/sys/devices/virtual/powercap\"],\"ReadonlyPaths\":[\"/proc/bus\",\"/proc/fs\",\"/proc/irq\",\"/proc/sys\",\"/proc/sysrq-trigger\"]},\"GraphDriver\":{\"Data\":{\"LowerDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b-init/diff:/var/lib/docker/overlay2/e54e72fb6f7adcbd7518162d9a51173724c6e0de6bdf6ef1792da02febe932c7/diff\",\"MergedDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b/merged\",\"UpperDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b/diff\",\"WorkDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b/work\"},\"Name\":\"overlay2\"},\"Mounts\":[],\"Config\":{\"Hostname\":\"dea8f8b6e087\",\"Domainname\":\"\",\"User\":\"\",\"AttachStdin\":false,\"AttachStdout\":true,\"AttachStderr\":true,\"Tty\":false,\"OpenStdin\":false,\"StdinOnce\":false,\"Env\":[\"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\"],\"Cmd\":[\"/hello\"],\"Image\":\"hello-world\",\"Volumes\":null,\"WorkingDir\":\"/\",\"Entrypoint\":null,\"OnBuild\":null,\"Labels\":{}},\"NetworkSettings\":{\"Bridge\":\"\",\"SandboxID\":\"\",\"SandboxKey\":\"\",\"Ports\":{},\"HairpinMode\":false,\"LinkLocalIPv6Address\":\"\",\"LinkLocalIPv6PrefixLen\":0,\"SecondaryIPAddresses\":null,\"SecondaryIPv6Addresses\":null,\"EndpointID\":\"\",\"Gateway\":\"\",\"GlobalIPv6Address\":\"\",\"GlobalIPv6PrefixLen\":0,\"IPAddress\":\"\",\"IPPrefixLen\":0,\"IPv6Gateway\":\"\",\"MacAddress\":\"\",\"Networks\":{\"bridge\":{\"IPAMConfig\":null,\"Links\":null,\"Aliases\":null,\"MacAddress\":\"\",\"DriverOpts\":null,\"NetworkID\":\"ded1ad71530e506e5f43499bf237695620374b2b4fb028cebdedf87a51d1fc7c\",\"EndpointID\":\"\",\"Gateway\":\"\",\"IPAddress\":\"\",\"IPPrefixLen\":0,\"IPv6Gateway\":\"\",\"GlobalIPv6Address\":\"\",\"GlobalIPv6PrefixLen\":0,\"DNSNames\":null}}}}".to_string();
}

#[cfg(test)]
fn get_processes_response() -> String {
    return "{\"Processes\":[[\"4586\",\"999\",\"rust\"]],\"Titles\":[\"PID\",\"USER\",\"COMMAND\"]}".to_string();
}

#[cfg(test)]
fn get_filesystem_changes_response() -> String {
    return "[{\"Path\":\"/tmp\",\"Kind\":0}]".to_string();
}

#[cfg(test)]
fn get_version_response() -> String {
    return "{\"Version\":\"1.8.1\",\"ApiVersion\":\"1.20\",\"GitCommit\":\"d12ea79\",\"GoVersion\":\"go1.4.2\",\"Os\":\"linux\",\"Arch\":\"amd64\",\"KernelVersion\":\"4.0.9-boot2docker\",\"BuildTime\":\"Thu Aug 13 02:49:29 UTC 2015\"}".to_string();
}
