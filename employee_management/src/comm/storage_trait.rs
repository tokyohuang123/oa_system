pub trait EmployeeStorage {
    fn insert(&mut self, employee: Employee) -> Result<(), String>;
    fn get(&self, id: u32) -> Option<&Employee>;
    // 其他数据存储相关的方法
}
