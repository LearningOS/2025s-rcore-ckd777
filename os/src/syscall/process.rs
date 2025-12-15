//! Process management syscalls
use crate::{
     task::{exit_current_and_run_next, suspend_current_and_run_next,add_yield_syscall_count}, timer::get_time_us
};
use crate::task::{get_syscall_times,add_trace_syscall_count,add_time_syscall_count};
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next(1);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    add_yield_syscall_count();
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    add_time_syscall_count();
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}
// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    add_trace_syscall_count();
    match _trace_request{
        0 => {
            let id = _id as *const u8;     // cast
            let value: u8 = unsafe { core::ptr::read_volatile(id) };
            return value as isize;
        }
        1=>{
            let ptr = _id as *mut u8;
            let value: u8 = (_data & 0xFF) as u8;   // 保留最低 8 位

            unsafe {
                // 向用户空间地址写 1 字节
                core::ptr::write_volatile(ptr, value);
            }

            return 0;
        }
        2=>{
            //print!("syscall times is {},id is {}\n",get_syscall_times(_id),_id);
            return get_syscall_times(_id);
        }
        _=>{
            trace!("kernel: sys_trace");
            return -1;
        }
    }
    
}
