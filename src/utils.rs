use log::warn;

pub fn read_fifo<T>(addr: usize) -> T {
    let addr = addr as *mut T;
    unsafe { addr.read_volatile() }
}

pub fn write_fifo<T>(addr: usize, val: T) {
    let addr = addr as *mut T;
    unsafe {
        addr.write_volatile(val);
    }
}

pub fn write_reg(addr: usize, val: u32) {
    let addr = addr as *mut u32;
    unsafe {
        addr.write_volatile(val);
    }
}

pub fn read_reg(addr: usize) -> u32 {
    let addr = addr as *mut u32;
    unsafe { addr.read_volatile() }
}

pub fn wait_ms<T>(_ms: usize, mut f: T)
where
    T: FnMut() -> bool,
{
    let mut timeout = 100_0000;
    while timeout > 0 && !f() {
        core::hint::spin_loop();
        timeout -= 1;
    }
    if timeout == 0 {
        warn!("wait timeout");
    }
}
