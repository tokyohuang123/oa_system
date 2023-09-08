mod comm;
// 使用模块中的内容
use comm::step::{WorkflowStep, WorkflowAction};
use comm::template::WorkflowTemplate;

// 公共接口和函数
pub fn add_workflow_step(template: &mut WorkflowTemplate, step: WorkflowStep) {
    template.steps.push(step);
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use comm::step::{WorkflowAction, WorkflowStep};
    use comm::template::WorkflowTemplate;

    #[test]
    fn test_add_workflow_step() {
        // 创建一个初始的 WorkflowTemplate
        let mut template = WorkflowTemplate {
            id: 1,
            name: "Test Template".to_string(),
            description: "A test template".to_string(),
            version: 1,
            steps: Vec::new(),
        };

        // 确保初始时没有步骤
        assert_eq!(template.steps.len(), 0);

        // 创建一个 WorkflowStep
        let step = WorkflowStep {
            id: 1,
            name: "Test Step".to_string(),
            description: "A test step".to_string(),
            order: 1,
            actions: vec![WorkflowAction::Approve, WorkflowAction::Reject],
        };

        // 使用 add_workflow_step 函数添加步骤
        add_workflow_step(&mut template, step);

        // 验证步骤已被添加
        assert_eq!(template.steps.len(), 1);
        assert_eq!(template.steps[0].name, "Test Step");
    }

    // 你已经有的测试
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
