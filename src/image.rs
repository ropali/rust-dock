#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Image {
    pub Created: u64,
    pub Id: String,
    pub ParentId: String,
    pub RepoTags: Vec<String>,
    pub Size: u64,
    pub VirtualSize: Option<u64>,
    pub Labels: serde_json::Value,
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let image = Image {
            Created: self.Created,
            Id: self.Id.clone(),
            ParentId: self.ParentId.clone(),
            RepoTags: self.RepoTags.clone(),
            Size: self.Size,
            VirtualSize: self.VirtualSize,
            Labels: self.Labels.clone(),
        };
        return image;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageStatus {
    pub status: Option<String>,
    pub error: Option<String>,
}

impl Clone for ImageStatus {
    fn clone(&self) -> Self {
        let image_status = ImageStatus {
            status: self.status.clone(),
            error: self.status.clone(),
        };
        return image_status;
    }
}


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ImageHistory {
    pub Id: String,
    pub Tags: Option<Vec<String>>,
    pub Size: u64,
    pub Created: u64,
    pub CreatedBy: String,
    pub Comment: String
}


impl Clone for ImageHistory {
    fn clone(&self) -> Self {
        return ImageHistory {
            Id: self.Id.clone(),
            Tags: self.Tags.clone(),
            Size: self.Size.clone(),
            Created: self.Created.clone(),
            CreatedBy: self.CreatedBy.clone(),
            Comment: self.Comment.clone()
        }
    }
}