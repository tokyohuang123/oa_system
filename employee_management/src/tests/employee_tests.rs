#[cfg(test)]
mod tests {
    use super::comm::models::Employee;
    use super::comm::service_trait::EmployeeService;
    use super::comm::storage_trait::EmployeeStorage;

    struct MockEmployeeStorage {
        employees: std::collections::HashMap<u32, Employee>,
    }

    impl MockEmployeeStorage {
        fn new() -> Self {
            MockEmployeeStorage {
                employees: std::collections::HashMap::new(),
            }
        }
    }

    impl EmployeeStorage for MockEmployeeStorage {
        fn insert(&mut self, employee: Employee) -> Result<(), String> {
            self.employees.insert(employee.get_id(), employee);
            Ok(())
        }

        fn get(&self, id: u32) -> Option<&Employee> {
            self.employees.get(&id)
        }
    }

    struct MockEmployeeService {
        storage: MockEmployeeStorage,
    }

    impl MockEmployeeService {
        fn new() -> Self {
            MockEmployeeService {
                storage: MockEmployeeStorage::new(),
            }
        }
    }

    impl EmployeeService for MockEmployeeService {
        fn add_employee(&mut self, employee: Employee) -> Result<(), String> {
            self.storage.insert(employee)
        }

        fn get_employee(&self, id: u32) -> Option<&Employee> {
            self.storage.get(id)
        }
    }

    #[test]
    fn test_add_employee() {
        let mut service = MockEmployeeService::new();
        let employee = Employee::new(1, "Alice".to_string(), "Engineer".to_string());
        assert!(service.add_employee(employee).is_ok());
    }

    #[test]
    fn test_get_employee() {
        let mut service = MockEmployeeService::new();
        let employee = Employee::new(1, "Alice".to_string(), "Engineer".to_string());
        service.add_employee(employee).unwrap();
        let retrieved_employee = service.get_employee(1);
        assert!(retrieved_employee.is_some());
        assert_eq!(retrieved_employee.unwrap().get_name(), "Alice");
    }
}
