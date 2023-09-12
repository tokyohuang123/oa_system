
use workflow_definition::comm::template::WorkflowTemplate;

// 定义WorkflowInstance结构体
pub struct WorkflowInstance {
 id: u32,
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

        // Getter 方法
        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_current_step(&self) -> usize {
            self.current_step
        }

        pub fn get_status(&self) -> &WorkflowStatus {
            &self.status
        }

        pub fn get_template(&self) -> &WorkflowTemplate {
            &self.template
        }

        // Setter 方法
        pub fn set_id(&mut self, id: u32) {
            self.id = id;
        }

        pub fn set_current_step(&mut self, current_step: usize) {
            self.current_step = current_step;
        }

        pub fn set_status(&mut self, status: WorkflowStatus) {
            self.status = status;
        }

        pub fn set_template(&mut self, template: WorkflowTemplate) {
            self.template = template;
        }


}
// 定义WorkflowStatus枚举
#[derive(Debug,PartialEq,Clone)]
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
        assert_eq!(*instance.get_status(), WorkflowStatus::Pending);
    }

    #[test]
    fn test_workflow_status_enum() {
        assert_eq!(WorkflowStatus::Pending as u32, 0);
        assert_eq!(WorkflowStatus::Running as u32, 1);
        // ... 其他状态测试
    }
}