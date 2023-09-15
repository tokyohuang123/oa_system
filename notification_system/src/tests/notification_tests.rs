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

    #[test]
    fn test_mark_notification_as_read() {
        let mut storage = InMemoryNotificationStorage {
            notifications: Default::default(),
        };

        let notification = Notification::new(1, "Test Title".to_string(), "Test Content".to_string(), 123);
        storage.save_notification(notification.clone());

        storage.mark_as_read(1).unwrap();
        let retrieved_notification = storage.get_notification(1).unwrap();
        assert_eq!(retrieved_notification.is_read, true);
    }

    #[test]
    fn test_get_all_unread_notifications() {
        let mut storage = InMemoryNotificationStorage {
            notifications: Default::default(),
        };

        let notification1 = Notification::new(1, "Title 1".to_string(), "Content 1".to_string(), 123);
        let notification2 = Notification::new(2, "Title 2".to_string(), "Content 2".to_string(), 124);
        let notification3 = Notification::new(3, "Title 3".to_string(), "Content 3".to_string(), 125);

        storage.save_notification(notification1.clone());
        storage.save_notification(notification2.clone());
        storage.save_notification(notification3.clone());

        storage.mark_as_read(2).unwrap();

        let unread_notifications = storage.get_all_unread_notifications();
        assert_eq!(unread_notifications.len(), 2);
        assert!(unread_notifications.iter().any(|n| n.id == 1));
        assert!(unread_notifications.iter().any(|n| n.id == 3));
    }

}
