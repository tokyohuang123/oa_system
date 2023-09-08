pub struct WorkflowTemplate {
   pub id: u32,
   pub name: String,
    pub description: String,
    pub version: u32,
    pub steps: Vec<crate::comm::step::WorkflowStep>, // 使用上级模块中的 WorkflowStep
}