use crate::comm::models::Document;
pub trait DocumentService {
    fn upload(&mut self, name: String, content: &[u8]) -> Result<(), String>;
    fn download(&self, id: u32) -> Option<&Document>;
    fn update_document(&mut self, doc: Document) -> Result<(), String>;
    fn delete_document(&mut self, id: u32) -> Result<(), String>;
    fn search_documents(&self, query: &str) -> Vec<Document>;
}
