pub struct WorkflowStep {
   pub id: u32,
   pub name: String,
   pub  description: String,
   pub  order: u32,
    pub actions: Vec<WorkflowAction>,
}
impl WorkflowStep {
    pub fn  new()->Self {
        Self{
            id:1,
            name: "Workflow".to_string(),
            order: 1,
            description: "Workflow".to_string(),
            actions: vec![WorkflowAction::Waiting],
        }
    }
    
}

pub enum WorkflowAction {
    Approve,
    Reject,
    Forward,
    Waiting,
    // ... 其他操作
}
