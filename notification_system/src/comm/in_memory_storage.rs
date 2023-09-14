use crate::comm::models::Notification;
use crate::comm::storage_trait::NotificationStorage;
use std::collections::HashMap;

pub struct InMemoryNotificationStorage {
    notifications: HashMap<u32, Notification>,
}

impl NotificationStorage for InMemoryNotificationStorage {
    fn save_notification(&mut self, notification: Notification) {
        self.notifications.insert(notification.get_id(), notification);
    }

    fn get_notification(&self, id: u32) -> Option<&Notification> {
        self.notifications.get(&id)
    }

    // ... other methods ...
}
