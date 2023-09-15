#[cfg(test)]
mod reporting_tests {
    use super::comm::models::Report;
    use super::comm::in_memory_storage::InMemoryReportStorage;
    use super::comm::storage_trait::ReportStorage;

    #[test]
    fn test_generate_report() {
        let mut storage = InMemoryReportStorage::new();

        let report = Report {
            id: 1,
            title: "Sample Report".to_string(),
            content: "This is a sample report content.".to_string(),
            date: "2023-09-10".to_string(),
        };

        storage.save_report(report.clone());
        let retrieved_report = storage.get_report(1).unwrap();
        assert_eq!(retrieved_report.title, "Sample Report");
    }
}
