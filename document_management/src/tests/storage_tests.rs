use crate::comm::models::{Document, Metadata};
use crate::comm::storage_trait::{Storage, InMemoryStorage};

#[test]
use crate::comm::models::Document;
use crate::comm::storage_trait::{Storage, InMemoryStorage};
fn test_save_and_get_document() {
    let mut storage = InMemoryStorage { documents: HashMap::new() };
    let doc = Document {
        id: 1,
        name: "Test Document".to_string(),
        path: "/path/to/doc".to_string(),
        format: "txt".to_string(),
        version: 1,
        metadata: Metadata {
            upload_date: "2023-09-08".to_string(),
            author: "star".to_string(),
            tags: vec!["tests".to_string()],
        },
    };

    storage.save(doc.clone()).unwrap();
    let retrieved_doc = storage.get(1).unwrap();
    assert_eq!(doc, *retrieved_doc);
}

// ... 添加其他测试，如更新、删除、搜索等
