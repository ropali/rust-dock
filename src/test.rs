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
    return "[{\"Name\":\"bridge\",\"Id\":\"f2de39df4171b0dc801e8002d1d999b77256983dfc63041c0f34030aa3977566\",\"Created\":\"2016-10-19T06:21:00.416543526Z\",\"Scope\":\"local\",\"Driver\":\"bridge\",\"EnableIPv6\":false,\"Internal\":false,\"Attachable\":false,\"Ingress\":false,\"IPAM\":{\"Driver\":\"default\",\"Config\":[{\"Subnet\":\"172.17.0.0/16\"}]},\"Options\":{\"com.docker.network.bridge.default_bridge\":\"true\",\"com.docker.network.bridge.enable_icc\":\"true\",\"com.docker.network.bridge.enable_ip_masquerade\":\"true\",\"com.docker.network.bridge.host_binding_ipv4\":\"0.0.0.0\",\"com.docker.network.bridge.name\":\"docker0\",\"com.docker.network.driver.mtu\":\"1500\"}}]".to_string();
}

#[cfg(test)]
fn get_containers_response() -> String {
    return "[{\"Id\":\"8dfafdbc3a40\",\"Names\":[\"/boring_feynman\"],\"Image\":\"ubuntu:latest\",\"ImageID\":\"d74508fb6632491cea586a1fd7d748dfc5274cd6fdfedee309ecdcbc2bf5cb82\",\"Command\":\"echo 1\",\"Created\":1367854155,\"State\":\"Exited\",\"Status\":\"Exit 0\",\"Ports\":[{\"PrivatePort\":2222,\"PublicPort\":3333,\"Type\":\"tcp\"}],\"Labels\":{\"com.example.vendor\":\"Acme\",\"com.example.license\":\"GPL\",\"com.example.version\":\"1.0\"},\"SizeRw\":12288,\"SizeRootFs\":0,\"HostConfig\":{\"NetworkMode\":\"default\",\"Annotations\":{\"io.kubernetes.docker.type\":\"container\"}},\"NetworkSettings\":{\"Networks\":{\"bridge\":{\"NetworkID\":\"7ea29fc1412292a2d7bba362f9253545fecdfa8ce9a6e37dd10ba8bee7129812\",\"EndpointID\":\"2cdc4edb1ded3631c81f57966563e5c8525b81121bb3706a9a9a3ae102711f3f\",\"Gateway\":\"172.17.0.1\",\"IPAddress\":\"172.17.0.2\",\"IPPrefixLen\":16,\"IPv6Gateway\":\"\",\"GlobalIPv6Address\":\"\",\"GlobalIPv6PrefixLen\":0,\"MacAddress\":\"02:42:ac:11:00:02\"}}},\"Mounts\":[{\"Name\":\"fac362...80535\",\"Source\":\"/data\",\"Destination\":\"/data\",\"Driver\":\"local\",\"Mode\":\"ro,Z\",\"RW\":false,\"Propagation\":\"\"}]}]".to_string();
}

#[cfg(test)]
fn get_stats_response() -> String {
    return "{\"read\":\"2015-01-08T22:57:31.547920715Z\",\"pids_stats\":{\"current\":3},\"networks\":{\"eth0\":{\"rx_bytes\":5338,\"rx_dropped\":0,\"rx_errors\":0,\"rx_packets\":36,\"tx_bytes\":648,\"tx_dropped\":0,\"tx_errors\":0,\"tx_packets\":8},\"eth5\":{\"rx_bytes\":4641,\"rx_dropped\":0,\"rx_errors\":0,\"rx_packets\":26,\"tx_bytes\":690,\"tx_dropped\":0,\"tx_errors\":0,\"tx_packets\":9}},\"memory_stats\":{\"stats\":{\"total_pgmajfault\":0,\"cache\":0,\"mapped_file\":0,\"total_inactive_file\":0,\"pgpgout\":414,\"rss\":6537216,\"total_mapped_file\":0,\"writeback\":0,\"unevictable\":0,\"pgpgin\":477,\"total_unevictable\":0,\"pgmajfault\":0,\"total_rss\":6537216,\"total_rss_huge\":6291456,\"total_writeback\":0,\"total_inactive_anon\":0,\"rss_huge\":6291456,\"hierarchical_memory_limit\":67108864,\"total_pgfault\":964,\"total_active_file\":0,\"active_anon\":6537216,\"total_active_anon\":6537216,\"total_pgpgout\":414,\"total_cache\":0,\"inactive_anon\":0,\"active_file\":0,\"pgfault\":964,\"inactive_file\":0,\"total_pgpgin\":477},\"max_usage\":6651904,\"usage\":6537216,\"failcnt\":0,\"limit\":67108864},\"blkio_stats\":{},\"cpu_stats\":{\"cpu_usage\":{\"percpu_usage\":[8646879,24472255,36438778,30657443],\"usage_in_usermode\":50000000,\"total_usage\":100215355,\"usage_in_kernelmode\":30000000},\"system_cpu_usage\":739306590000000,\"online_cpus\":4,\"throttling_data\":{\"periods\":0,\"throttled_periods\":0,\"throttled_time\":0}},\"precpu_stats\":{\"cpu_usage\":{\"percpu_usage\":[8646879,24350896,36438778,30657443],\"usage_in_usermode\":50000000,\"total_usage\":100093996,\"usage_in_kernelmode\":30000000},\"system_cpu_usage\":9492140000000,\"online_cpus\":4,\"throttling_data\":{\"periods\":0,\"throttled_periods\":0,\"throttled_time\":0}}}".to_string();
}

#[cfg(test)]
fn get_system_info_response() -> String {
    return "{\"Containers\":6,\"Debug\":0,\"DockerRootDir\":\"/var/lib/docker\",\"Driver\":\"btrfs\",\"DriverStatus\":[[\"Build Version\",\"Btrfs v3.17.1\"],[\"Library Version\",\"101\"]],\"ExecutionDriver\":\"native-0.2\",\"ID\":\"WG63:3NIU:TSI2:FV7J:IL2O:YPXA:JR3F:XEKT:JZVR:JA6T:QMYE:B4SB\",\"IPv4Forwarding\":1,\"Images\":190,\"IndexServerAddress\":\"https://index.docker.io/v1/\",\"InitPath\":\"/usr/libexec/docker/dockerinit\",\"InitSha1\":\"30c93967bdc3634b6036e1a76fd547bbe171b264\",\"KernelVersion\":\"3.18.6\",\"Labels\":null,\"MemTotal\":16854257664,\"MemoryLimit\":1,\"NCPU\":4,\"NEventsListener\":0,\"NFd\":68,\"NGoroutines\":95,\"Name\":\"core\",\"OperatingSystem\":\"CoreOS 607.0.0\",\"RegistryConfig\":{\"IndexConfigs\":{\"docker.io\":{\"Mirrors\":null,\"Name\":\"docker.io\",\"Official\":true,\"Secure\":true}},\"InsecureRegistryCIDRs\":[\"127.0.0.0/8\"]},\"SwapLimit\":1}".to_string();
}

#[cfg(test)]
fn get_images_response() -> String {
    return "[{\"Id\":\"sha256:ec3f0931a6e6b6855d76b2d7b0be30e81860baccd891b2e243280bf1cd8ad710\",\"ParentId\":\"\",\"RepoTags\":[\"example:1.0\",\"example:latest\",\"example:stable\",\"internal.registry.example.com:5000/example:1.0\"],\"RepoDigests\":[\"example@sha256:afcc7f1ac1b49db317a7196c902e61c6c3c4607d63599ee1a82d702d249a0ccb\",\"internal.registry.example.com:5000/example@sha256:b69959407d21e8a062e0416bf13405bb2b71ed7a84dde4158ebafacfa06f5578\"],\"Created\":1644009612,\"Size\":172064416,\"SharedSize\":1239828,\"VirtualSize\":172064416,\"Labels\":{\"com.example.some-label\":\"some-value\",\"com.example.some-other-label\":\"some-other-value\"},\"Containers\":2}]".to_string();
}

#[cfg(test)]
fn get_container_info_response() -> String {
    return "{\"Id\":\"dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118\",\"Created\":\"2024-06-02T14:56:43.3912062Z\",\"Path\":\"/hello\",\"Args\":[],\"State\":{\"Status\":\"exited\",\"Running\":false,\"Paused\":false,\"Restarting\":false,\"OOMKilled\":false,\"Dead\":false,\"Pid\":0,\"ExitCode\":0,\"Error\":\"\",\"StartedAt\":\"2024-07-14T09:15:09.506227996Z\",\"FinishedAt\":\"2024-07-14T09:15:09.637925962Z\"},\"Image\":\"sha256:d2c94e258dcb3c5ac2798d32e1249e42ef01cba4841c2234249495f87264ac5a\",\"ResolvConfPath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/resolv.conf\",\"HostnamePath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/hostname\",\"HostsPath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/hosts\",\"LogPath\":\"/var/lib/docker/containers/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118/dea8f8b6e087c4e63816cf2c987105b361cdb7c27b4c78910f8e94abfe628118-json.log\",\"Name\":\"/lucid_roentgen\",\"RestartCount\":0,\"Driver\":\"overlay2\",\"Platform\":\"linux\",\"MountLabel\":\"\",\"ProcessLabel\":\"\",\"AppArmorProfile\":\"docker-default\",\"ExecIDs\":null,\"HostConfig\":{\"Binds\":null,\"ContainerIDFile\":\"\",\"LogConfig\":{\"Type\":\"json-file\",\"Config\":{}},\"NetworkMode\":\"bridge\",\"PortBindings\":{},\"RestartPolicy\":{\"Name\":\"no\",\"MaximumRetryCount\":0},\"AutoRemove\":false,\"VolumeDriver\":\"\",\"VolumesFrom\":null,\"ConsoleSize\":[28,90],\"CapAdd\":null,\"CapDrop\":null,\"CgroupnsMode\":\"private\",\"Dns\":[],\"DnsOptions\":[],\"DnsSearch\":[],\"ExtraHosts\":null,\"GroupAdd\":null,\"IpcMode\":\"private\",\"Cgroup\":\"\",\"Links\":null,\"OomScoreAdj\":0,\"PidMode\":\"\",\"Privileged\":false,\"PublishAllPorts\":false,\"ReadonlyRootfs\":false,\"SecurityOpt\":null,\"UTSMode\":\"\",\"UsernsMode\":\"\",\"ShmSize\":67108864,\"Runtime\":\"runc\",\"Isolation\":\"\",\"CpuShares\":0,\"Memory\":0,\"NanoCpus\":0,\"CgroupParent\":\"\",\"BlkioWeight\":0,\"BlkioWeightDevice\":[],\"BlkioDeviceReadBps\":[],\"BlkioDeviceWriteBps\":[],\"BlkioDeviceReadIOps\":[],\"BlkioDeviceWriteIOps\":[],\"CpuPeriod\":0,\"CpuQuota\":0,\"CpuRealtimePeriod\":0,\"CpuRealtimeRuntime\":0,\"CpusetCpus\":\"\",\"CpusetMems\":\"\",\"Devices\":[],\"DeviceCgroupRules\":null,\"DeviceRequests\":null,\"MemoryReservation\":0,\"MemorySwap\":0,\"MemorySwappiness\":null,\"OomKillDisable\":null,\"PidsLimit\":null,\"Ulimits\":[],\"CpuCount\":0,\"CpuPercent\":0,\"IOMaximumIOps\":0,\"IOMaximumBandwidth\":0,\"MaskedPaths\":[\"/proc/asound\",\"/proc/acpi\",\"/proc/kcore\",\"/proc/keys\",\"/proc/latency_stats\",\"/proc/timer_list\",\"/proc/timer_stats\",\"/proc/sched_debug\",\"/proc/scsi\",\"/sys/firmware\",\"/sys/devices/virtual/powercap\"],\"ReadonlyPaths\":[\"/proc/bus\",\"/proc/fs\",\"/proc/irq\",\"/proc/sys\",\"/proc/sysrq-trigger\"]},\"GraphDriver\":{\"Data\":{\"LowerDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b-init/diff:/var/lib/docker/overlay2/e54e72fb6f7adcbd7518162d9a51173724c6e0de6bdf6ef1792da02febe932c7/diff\",\"MergedDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b/merged\",\"UpperDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b/diff\",\"WorkDir\":\"/var/lib/docker/overlay2/538d1b75663d2aa2dea6ffcbb8c2a9222f7da6ec864add2fb6c358943baf136b/work\"},\"Name\":\"overlay2\"},\"Mounts\":[],\"Config\":{\"Hostname\":\"dea8f8b6e087\",\"Domainname\":\"\",\"User\":\"\",\"AttachStdin\":false,\"AttachStdout\":true,\"AttachStderr\":true,\"Tty\":false,\"OpenStdin\":false,\"StdinOnce\":false,\"Env\":[\"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\"],\"Cmd\":[\"/hello\"],\"Image\":\"hello-world\",\"Volumes\":null,\"WorkingDir\":\"/\",\"Entrypoint\":null,\"OnBuild\":null,\"Labels\":{}},\"NetworkSettings\":{\"Bridge\":\"\",\"SandboxID\":\"\",\"SandboxKey\":\"\",\"Ports\":{},\"HairpinMode\":false,\"LinkLocalIPv6Address\":\"\",\"LinkLocalIPv6PrefixLen\":0,\"SecondaryIPAddresses\":null,\"SecondaryIPv6Addresses\":null,\"EndpointID\":\"\",\"Gateway\":\"\",\"GlobalIPv6Address\":\"\",\"GlobalIPv6PrefixLen\":0,\"IPAddress\":\"\",\"IPPrefixLen\":0,\"IPv6Gateway\":\"\",\"MacAddress\":\"\",\"Networks\":{\"bridge\":{\"IPAMConfig\":null,\"Links\":null,\"Aliases\":null,\"MacAddress\":\"\",\"DriverOpts\":null,\"NetworkID\":\"ded1ad71530e506e5f43499bf237695620374b2b4fb028cebdedf87a51d1fc7c\",\"EndpointID\":\"\",\"Gateway\":\"\",\"IPAddress\":\"\",\"IPPrefixLen\":0,\"IPv6Gateway\":\"\",\"GlobalIPv6Address\":\"\",\"GlobalIPv6PrefixLen\":0,\"DNSNames\":null}}}}".to_string();
}

#[cfg(test)]
fn get_processes_response() -> String {
    return "{\"Titles\":[\"UID\",\"PID\",\"PPID\",\"C\",\"STIME\",\"TTY\",\"TIME\",\"CMD\"],\"Processes\":[[\"root\",\"13642\",\"882\",\"0\",\"17:03\",\"pts/0\",\"00:00:00\",\"/bin/bash\"],[\"root\",\"13735\",\"13642\",\"0\",\"17:06\",\"pts/0\",\"00:00:00\",\"sleep 10\"]]}".to_string();
}

#[cfg(test)]
fn get_filesystem_changes_response() -> String {
    return "[{\"Path\":\"/dev\",\"Kind\":0},{\"Path\":\"/dev/kmsg\",\"Kind\":1},{\"Path\":\"/test\",\"Kind\":1}]".to_string();
}

#[cfg(test)]
fn get_version_response() -> String {
    return "{\"Platform\":{\"Name\":\"string\"},\"Components\":[{\"Name\":\"Engine\",\"Version\":\"19.03.12\",\"Details\":{}}],\"Version\":\"19.03.12\",\"ApiVersion\":\"1.40\",\"MinAPIVersion\":\"1.12\",\"GitCommit\":\"48a66213fe\",\"GoVersion\":\"go1.13.14\",\"Os\":\"linux\",\"Arch\":\"amd64\",\"KernelVersion\":\"4.19.76-linuxkit\",\"Experimental\":true,\"BuildTime\":\"2020-06-22T15:49:27.000000000+00:00\"}".to_string();
}
