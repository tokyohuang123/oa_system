// 在workflow_instance/src/lib.rs中

mod comm;

pub use comm::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // 这里可以添加一些库级别的集成测试
        // 例如，测试comm模块中的api和models之间的交互
        // 请根据实际的功能和需求填写具体的测试内容
    }
}
