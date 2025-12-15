//! Process management syscalls
use crate::task::{current_user_token,change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,mmap};
use crate::mm::{translated_byte_buffer};
use crate::config::PAGE_SIZE;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = crate::timer::get_time_us();
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, core::mem::size_of::<TimeVal>());
    let tv = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let src = &tv as *const _ as *const u8;
    let mut offset = 0;
    for buf in buffers {
    let len = buf.len();
    buf.copy_from_slice(unsafe {
        core::slice::from_raw_parts(src.add(offset), len)
    });
    offset += len;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match  _trace_request {
        0 => {
            // trace_read
            

            -1
        }
        1 => {
            // trace_write
            
            
            -1
        }
        2 => {
            // trace_get_count
            

            -1
        }
        _ => {
            trace!("kernel: sys_trace invalid trace_request {}", _trace_request);
            -1
        }
    }
    
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 {
        trace!("kernel: sys_mmap start address not aligned by page size");
        return -1;
    }
    if _port& !0x7 != 0 || _port & 0x7 == 0 {
        trace!("kernel: sys_mmap invalid prot {}", _port);
        return -1;
    }
    mmap(_start, _len, _port as u8)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
