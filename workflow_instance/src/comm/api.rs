// workflow_instance/src/api.rs

use super::models::{WorkflowInstance, WorkflowStatus};
use workflow_definition::comm::template::WorkflowTemplate;
use crate::comm::storage::Storage;
use crate::comm::storage::InMemoryStorage;

// 提供API来创建、查询和管理工作流实例
pub fn create_instance(template: WorkflowTemplate) -> WorkflowInstance {
    WorkflowInstance::new(template)
}

pub fn get_instance_status<S: Storage>(storage: &S, instance_id:u32) -> WorkflowStatus {
    match storage.get_instance_status(instance_id) {
        Some(status) => status,
        None => WorkflowStatus::Pending,
    }
}

// ... 其他API函数

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instance() {
        let template = WorkflowTemplate::new(); // 假设WorkflowTemplate有一个new方法
        let instance = create_instance(template);

        assert_eq!(*instance.get_status(), WorkflowStatus::Pending);
        // ... 其他断言
    }

    #[test]
    fn test_get_instance_status() {
        let mut storage = InMemoryStorage::new();
        let template = WorkflowTemplate::new(); // 假设WorkflowTemplate有一个new方法
        let instance = WorkflowInstance::new(template); // 提供正确的参数
        storage.insert(instance);
        let status = get_instance_status(&storage, 1);
        assert_eq!(status, WorkflowStatus::Pending);
        // ... 其他断言
    }
}
