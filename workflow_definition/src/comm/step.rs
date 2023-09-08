pub struct WorkflowStep {
   pub id: u32,
   pub name: String,
   pub  description: String,
   pub  order: u32,
    pub actions: Vec<WorkflowAction>,
}

pub enum WorkflowAction {
    Approve,
    Reject,
    Forward,
    // ... 其他操作
}
