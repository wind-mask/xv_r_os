use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            #[allow(deprecated)]
            console_putchar(c as usize); //下面的实现在从user space返回后打印存在错误，原因不明
        }
        // let addr = s.as_bytes().as_ptr() as usize;
        // let addr_low = addr as u32;
        // let addr_high = (addr >> 32) as u32;
        // let s = Physical::new(s.as_bytes().len(), addr_low as usize, addr_high as usize);
        // console_write(s);

        Ok(())
    }
}

// #[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::printf::_print(format_args!($fmt $(, $($arg)+)?))

    }
}
// #[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::printf::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))

    }
}
pub(crate) use print;
pub(crate) use println;
pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
#[deprecated]
#[allow(unused)]
#[allow(deprecated)]
fn console_putchar(c: usize) {
    sbi_rt::legacy::console_putchar(c);
}
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
