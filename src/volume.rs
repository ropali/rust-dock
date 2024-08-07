#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Volume {
    pub Name: String,
    pub Driver: String,
    pub Mountpoint: String,
    pub CreatedAt: String,
    pub Status: Option<serde_json::Value>,
    pub Labels: serde_json::Value,
    pub Scope: String,
    pub ClusterVolume: Option<serde_json::Value>,
    pub Options: Option<serde_json::Value>,
    pub UsageData: Option<serde_json::Value>
}


#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct VolumesResponse {
    pub Volumes: Vec<Volume>,
    pub Warnings: Option<Vec<String>>
}