use crate::comm::models::{Chart, Report};
pub trait ReportingService {
    fn generate_report(&self, report: &Report) -> Result<(), String>;
    fn update_report(&mut self, report: &Report) -> Result<(), String>;
    fn delete_report(&self, report_id: u32) -> Result<(), String>;
    fn list_reports(&self) -> Vec<Report>;

    // ... 其他服务方法 ...
}

pub trait ChartService {
    fn create_chart(&self, chart: &Chart) -> Result<(), String>;
    fn update_chart(&mut self, chart: &Chart) -> Result<(), String>;
    fn delete_chart(&self, chart_id: u32) -> Result<(), String>;
    fn list_charts(&self) -> Vec<Chart>;

    // ... 其他服务方法 ...
}
