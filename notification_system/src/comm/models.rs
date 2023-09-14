#[derive(Debug, Clone)]
pub struct Notification {
    id: u32,
    title: String,
    content: String,
    recipient_id: u32,
    is_read: bool,
}

impl Notification {
    pub fn new(id: u32, title: String, content: String, recipient_id: u32) -> Self {
        Notification {
            id,
            title,
            content,
            recipient_id,
            is_read: false,
        }
    }


        // Getter for id
        pub fn get_id(&self) -> u32 {
            self.id
        }

        // Setter for id
        pub fn set_id(&mut self, id: u32) {
            self.id = id;
        }

        // Getter for title
        pub fn get_title(&self) -> &String {
            &self.title
        }

        // Setter for title
        pub fn set_title(&mut self, title: String) {
            self.title = title;
        }

        // Getter for content
        pub fn get_content(&self) -> &String {
            &self.content
        }

        // Setter for content
        pub fn set_content(&mut self, content: String) {
            self.content = content;
        }

        // Getter for recipient_id
        pub fn get_recipient_id(&self) -> u32 {
            self.recipient_id
        }

        // Setter for recipient_id
        pub fn set_recipient_id(&mut self, recipient_id: u32) {
            self.recipient_id = recipient_id;
        }

        // Getter for is_read
        pub fn is_read(&self) -> bool {
            self.is_read
        }

        // Setter for is_read
        pub fn set_read(&mut self, is_read: bool) {
            self.is_read = is_read;
        }
    }

