use crate::comm::models::Notification;

pub trait NotificationService {
    fn send_notification(&mut self, title: String, content: String, recipient_id: u32);
    fn mark_as_read(&mut self, notification_id: u32);
    // ... other service-related methods ...
}
