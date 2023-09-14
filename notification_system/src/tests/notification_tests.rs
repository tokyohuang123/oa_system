#[cfg(test)]
mod tests {
    use super::comm::models::Notification;
    use super::comm::in_memory_storage::InMemoryNotificationStorage;
    use super::comm::storage_trait::NotificationStorage;

    #[test]
    fn test_save_and_get_notification() {
        let mut storage = InMemoryNotificationStorage {
            notifications: Default::default(),
        };

        let notification = Notification::new(1, "Test Title".to_string(), "Test Content".to_string(), 123);

        storage.save_notification(notification.clone());

        let retrieved_notification = storage.get_notification(1).unwrap();

        assert_eq!(retrieved_notification.title, "Test Title");
        assert_eq!(retrieved_notification.content, "Test Content");
        assert_eq!(retrieved_notification.recipient_id, 123);
        assert_eq!(retrieved_notification.is_read, false);
    }

    // ... 更多的测试 ...
}
