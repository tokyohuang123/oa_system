use std::collections::HashMap;
use crate::comm::models::Document;

pub trait Storage {
    fn save(&mut self, doc: Document) -> Result<(), String>;
    fn get(&self, id: u32) -> Option<&Document>;
    fn update(&mut self, doc: Document) -> Result<(), String>;
    fn delete(&mut self, id: u32) -> Result<(), String>;
    fn search(&self, query: &str) -> Vec<Document>;
}
pub struct InMemoryStorage {
    documents: HashMap<u32, Document>,
}

impl Storage for InMemoryStorage {
    fn save(&mut self, doc: Document) -> Result<(), String> {
        self.documents.insert(doc.get_id(), doc);
        Ok(())
    }

    fn get(&self, id: u32) -> Option<&Document> {
        self.documents.get(&id)
    }

    fn update(&mut self, doc: Document) -> Result<(), String> {
        if self.documents.contains_key(&doc.get_id()) {
            self.documents.insert(doc.get_id(), doc);
            Ok(())
        } else {
            Err("Document not found".to_string())
        }
    }

    fn delete(&mut self, id: u32) -> Result<(), String> {
        self.documents.remove(&id);
        Ok(())
    }

    fn search(&self, query: &str) -> Vec<Document> {
        self.documents.values()
            .filter(|&doc| doc.get_name().contains(query))
            .cloned()
            .collect()
    }
}
