#[derive(Debug, Clone, PartialEq)]  
pub struct Document {  
    id: u32,  
    name: String,  
    path: String,  
    format: String,  
    version: u32,  
    metadata: Metadata,  
}  
  
impl Document {  
    pub fn get_id(&self) -> u32 {  
        self.id  
    }  
  
    pub fn get_name(&self) -> &String {  
        &self.name  
    }  
  
    pub fn get_path(&self) -> &String {  
        &self.path  
    }  
  
    pub fn get_format(&self) -> &String {  
        &self.format  
    }  
  
    pub fn get_version(&self) -> u32 {  
        self.version  
    }  
  
    pub fn get_metadata(&self) -> &Metadata {  
        &self.metadata  
    }  
    pub fn set_id(&mut self, id: u32) {  
        self.id = id;  
    }  
  
    pub fn set_name(&mut self, name: String) {  
        self.name = name;  
    }  
  
    pub fn set_path(&mut self, path: String) {  
        self.path = path;  
    }  
  
    pub fn set_format(&mut self, format: String) {  
        self.format = format;  
    }  
  
    pub fn set_version(&mut self, version: u32) {  
        self.version = version;  
    }  
  
    pub fn set_metadata(&mut self, metadata: Metadata) {  
        self.metadata = metadata;  
    }  
}

#[derive(Debug, Clone, PartialEq)]  
pub struct Metadata {  
    upload_date: String, // 使用日期时间类型更佳  
    author: String,  
    tags: Vec<String>,  
}  
  
impl Metadata {  
    // Getter for upload_date  
    pub fn get_upload_date(&self) -> &String {  
        &self.upload_date  
    }  
    // Setter for upload_date  
    pub fn set_upload_date(&mut self, upload_date: String) {  
        self.upload_date = upload_date;  
    }  
    // Getter for author  
    pub fn get_author(&self) -> &String {  
        &self.author  
    }  
    // Setter for author  
    pub fn set_author(&mut self, author: String) {  
        self.author = author;  
    }  
    // Getter for tags  
    pub fn get_tags(&self) -> &Vec<String> {  
        &self.tags  
    }  
    // Setter for tags  
    pub fn set_tags(&mut self, tags: Vec<String>) {  
        self.tags = tags;  
    }  
}