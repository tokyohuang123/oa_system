pub trait EmployeeService {
    fn add_employee(&mut self, employee: Employee) -> Result<(), String>;
    fn get_employee(&self, id: u32) -> Option<&Employee>;
    // 其他员工管理相关的方法
}
