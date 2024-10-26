use crate::println;

pub fn test_runner(_test: &[&dyn Fn()]) {
    println!("Running {} tests", _test.len());
    for test in _test {
        test();
    }
    println!("{} tests passed", _test.len());
}
