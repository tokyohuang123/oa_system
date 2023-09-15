use std::collections::HashMap;
use crate::comm::models::{Chart, Report};
use crate::comm::storage_trait::{ChartStorage, ReportStorage};

pub struct InMemoryChartStorage {
    charts: HashMap<u32, Chart>,
}

impl InMemoryChartStorage {
    pub fn new() -> Self {
        InMemoryChartStorage {
            charts: HashMap::new(),
        }
    }
}

impl ChartStorage for InMemoryChartStorage {
    fn save_chart(&mut self, chart: &Chart) -> Result<(), String>{
        self.charts.insert(chart.get_id(), chart.clone());
        Ok(())
    }

    fn get_chart(&self, id: u32) -> Option<Chart> {
        self.charts.get(&id).cloned()
    }
    fn update_chart(&mut self, chart: &Chart) -> Result<(), String> {
        self.charts.insert(chart.get_id(), chart.clone());
        Ok(())
    }

    fn delete_chart(&mut self, chart_id: u32) -> Result<(), String> {
        self.charts.remove(&chart_id);
        Ok(())
    }

    fn list_charts(&self) -> Vec<Chart> {
        self.charts.values().cloned().collect()
    }
}

pub struct InMemoryReportStorage {
    reports: HashMap<u32, Report>,
}

impl InMemoryReportStorage {
    pub fn new() -> Self {
        InMemoryReportStorage {
            reports: HashMap::new(),
        }
    }
}

impl ReportStorage for InMemoryReportStorage {
    fn save_report(&mut self, report: &Report)-> Result<(), String> {
        self.reports.insert(report.get_id(), report.clone());
        Ok(())
    }

    fn get_report(&self, id: u32) -> Option<Report> {
        self.reports.get(&id).cloned()
    }
    fn update_report(&mut self, report: &Report) -> Result<(), String> {
        self.reports.insert(report.get_id(), report.clone());
        Ok(())
    }

    fn delete_report(&mut self, report_id: u32) -> Result<(), String> {
        self.reports.remove(&report_id);
        Ok(())
    }

    fn list_reports(&self) -> Vec<Report> {
        self.reports.values().cloned().collect()
    }
}
