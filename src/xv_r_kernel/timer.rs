use riscv::register::time;
use sbi_rt::set_timer;

pub fn get_time() -> usize {
    time::read()
}
const MICRO_PER_SEC: usize = 1_000_000;

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}
pub fn get_time_s() -> usize {
    time::read() / (CLOCK_FREQ)
}
use crate::board::qemu::CLOCK_FREQ;
const TICKS_PER_SEC: usize = 20;

pub fn set_next_trigger() {
    set_timer(
        (get_time() + CLOCK_FREQ / TICKS_PER_SEC)
            .try_into()
            .unwrap(),
    ); // 1000/20 = 50ms
}
