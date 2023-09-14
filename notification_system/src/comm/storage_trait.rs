use crate::comm::models::Notification;

pub trait NotificationStorage {
    fn save_notification(&mut self, notification: Notification);
    fn get_notification(&self, id: u32) -> Option<&Notification>;
    // ... other storage-related methods ...
}
