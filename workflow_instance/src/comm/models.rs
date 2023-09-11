
use workflow_definition::comm::template::WorkflowTemplate;

// 定义WorkflowInstance结构体
pub struct WorkflowInstance {
   pub id: u32,
    current_step: usize,
    status: WorkflowStatus,
    template: WorkflowTemplate,
    // ... 其他字段
}
impl WorkflowInstance {
    pub fn new(template: WorkflowTemplate)->WorkflowInstance {
        Self{
            id:1,
            current_step:0,
            status:WorkflowStatus::Pending,
            template:template
        }
    }
    pub fn status(&self)->&WorkflowStatus {
        &self.status
}
    pub fn current_step(&self)->usize {
        self.current_step
    }
}
// 定义WorkflowStatus枚举
#[derive(Debug,PartialEq)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    // ... 其他状态
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_instance_creation() {
        let template = WorkflowTemplate::new(); // 假设WorkflowTemplate有一个new方法
        let instance = WorkflowInstance {
            id: 1,
            current_step: 0,
            status: WorkflowStatus::Pending,
            template,
        };

        assert_eq!(instance.id, 1);
        assert_eq!(instance.current_step, 0);
        assert_eq!(instance.status(), WorkflowStatus::Pending);
    }

    #[test]
    fn test_workflow_status_enum() {
        assert_eq!(WorkflowStatus::Pending as u32, 0);
        assert_eq!(WorkflowStatus::Running as u32, 1);
        // ... 其他状态测试
    }
}