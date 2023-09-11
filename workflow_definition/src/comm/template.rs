pub struct WorkflowTemplate {
   pub id: u32,
   pub name: String,
    pub description: String,
    pub version: u32,
    pub steps: Vec<crate::comm::step::WorkflowStep>, // 使用上级模块中的 WorkflowStep
}

impl WorkflowTemplate {
    pub fn new()->WorkflowTemplate {
        WorkflowTemplate { 
            id: 1, 
            name: "WorkflowTemplate".to_string(), 
            description: "Workflow template".to_string(),
            version: 1, 
            steps: vec![crate::comm::step::WorkflowStep::new()]
        }
    }
}