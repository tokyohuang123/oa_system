// 在workflow_instance/src/api.rs中

use super::models::{WorkflowInstance, WorkflowStatus};
use workflow_definition::comm::template::WorkflowTemplate;

// 提供API来创建、查询和管理工作流实例
pub fn create_instance(template: WorkflowTemplate) -> WorkflowInstance {
    WorkflowInstance::new(template)

}
pub fn get_instance_status(instance_id: u32) -> WorkflowStatus {
    WorkflowStatus::Pending
}

// ... 其他API函数


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instance() {
        let template = WorkflowTemplate::new(); // 假设WorkflowTemplate有一个new方法
        let instance = create_instance(template);

        assert_eq!(instance.status(), WorkflowStatus::Pending);
        // ... 其他断言
    }

    #[test]
    fn test_get_instance_status() {
        let status = get_instance_status(1); // 假设有一个ID为1的实例
        assert_eq!(status, WorkflowStatus::Pending);
        // ... 其他断言
    }
}
