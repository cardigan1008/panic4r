use std::env;
use vte::{Parser, Perform};

struct TestPerform;

impl Perform for TestPerform {
    fn print(&mut self, _c: char) {}
    fn execute(&mut self, _byte: u8) {}
    fn hook(&mut self, _params: &[i64], _intermediate: &[u8], _ignored: bool, _c: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn csi_dispatch(
        &mut self,
        _params: &[i64],
        _intermediate: &[u8],
        _ignored: bool,
        _c: char,
    ) {
    }
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignored: bool, _byte: u8) {}
}

fn main() {

    let mut parser = Parser::new();
    let mut perform = TestPerform;

    // 读取文件内容
    let data = std::fs::read("test.vte").expect("Failed to read test file");

    // 处理数据
    for byte in data {
        parser.advance(&mut perform, byte);
    }
}
