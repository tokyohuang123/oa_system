// comm/storage.rs
use crate::models::WorkflowStatus;
use crate::models::WorkflowInstance;
use std::collections::HashMap;

pub trait Storage {
    fn get_instance(&self, instance_id: u32) -> Option<&WorkflowInstance>;
    fn get_instance_status(&self, instance_id: u32) -> Option<WorkflowStatus>;
}

pub struct InMemoryStorage {
    instances: HashMap<u32, WorkflowInstance>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage {
            instances: HashMap::new(),
        }
    }

    pub fn insert(&mut self, instance: WorkflowInstance) {
        self.instances.insert(instance.get_id(), instance);
    }
}

impl Storage for InMemoryStorage {
    fn get_instance(&self, instance_id: u32) -> Option<&WorkflowInstance> {
        self.instances.get(&instance_id)
    }

    fn get_instance_status(&self, instance_id: u32) -> Option<WorkflowStatus> {
        self.instances.get(&instance_id).map(|instance| instance.get_status().clone())
    }
}
