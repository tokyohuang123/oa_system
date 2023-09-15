use crate::comm::models::{Chart, Report};

pub trait ReportStorage {
    fn save_report(&mut self, report: &Report) -> Result<(), String>;
    fn get_report(&self, report_id: u32) -> Option<Report>;
    fn update_report(&mut self, report: &Report) -> Result<(), String>;
    fn delete_report(&mut self, report_id: u32) -> Result<(), String>;
    fn list_reports(&self) -> Vec<Report>;

    // ... 其他存储方法 ...
}

pub trait ChartStorage {
    fn save_chart(&mut self, chart: &Chart) -> Result<(), String>;
    fn get_chart(&self, chart_id: u32) -> Option<Chart>;
    fn update_chart(&mut self, chart: &Chart) -> Result<(), String>;
    fn delete_chart(&mut self, chart_id: u32) -> Result<(), String>;
    fn list_charts(&self) -> Vec<Chart>;

    // ... 其他存储方法 ...
}
