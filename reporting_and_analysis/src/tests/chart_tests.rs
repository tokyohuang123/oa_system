#[cfg(test)]
mod chart_tests {
    use super::comm::models::Chart;
    use super::comm::in_memory_storage::InMemoryChartStorage;
    use super::comm::storage_trait::ChartStorage;

    #[test]
    fn test_create_chart() {
        let mut storage = InMemoryChartStorage::new();

        let chart = Chart {
            id: 1,
            title: "Sample Chart".to_string(),
            data: vec![1.0, 2.0, 3.0],
            chart_type: "Line".to_string(),
        };

        storage.save_chart(chart.clone());
        let retrieved_chart = storage.get_chart(1).unwrap();
        assert_eq!(retrieved_chart.title, "Sample Chart");
    }
}
