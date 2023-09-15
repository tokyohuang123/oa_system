#[derive(Debug, Clone,PartialEq)]
pub struct Report {
    id: u32,
    title: String,
    content: String,
    date_generated: String,
    author: String,
    status: String,
    comments: Vec<String>,
}

impl Report {
    pub fn new(id: u32, title: String, content: String, date_generated: String, author: String, status: String) -> Self {
        Report {
            id,
            title,
            content,
            date_generated,
            author,
            status,
            comments: Vec::new(),
        }
    }
    // Getter methods for Report
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_date_generated(&self) -> &String {
        &self.date_generated
    }

    pub fn get_author(&self) -> &String {
        &self.author
    }

    pub fn get_status(&self) -> &String {
        &self.status
    }

    pub fn get_comments(&self) -> &Vec<String> {
        &self.comments
    }

    // Setter methods for Report
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_date_generated(&mut self, date_generated: String) {
        self.date_generated = date_generated;
    }

    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }
}
#[derive(Debug, Clone,PartialEq)]
pub struct Chart {
    id: u32,
    chart_type: String,
    data_source: String,
    description: String,
    created_by: String,
    last_updated: String,
}

impl Chart {
    pub fn new(id: u32, chart_type: String, data_source: String, description: String, created_by: String, last_updated: String) -> Self {
        Chart {
            id,
            chart_type,
            data_source,
            description,
            created_by,
            last_updated,
        }
    }
    // Getter methods for Chart
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_chart_type(&self) -> &String {
        &self.chart_type
    }

    pub fn get_data_source(&self) -> &String {
        &self.data_source
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_created_by(&self) -> &String {
        &self.created_by
    }

    pub fn get_last_updated(&self) -> &String {
        &self.last_updated
    }

    // Setter methods for Chart
    pub fn set_chart_type(&mut self, chart_type: String) {
        self.chart_type = chart_type;
    }

    pub fn set_data_source(&mut self, data_source: String) {
        self.data_source = data_source;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_created_by(&mut self, created_by: String) {
        self.created_by = created_by;
    }

    pub fn set_last_updated(&mut self, last_updated: String) {
        self.last_updated = last_updated;
    }
}
