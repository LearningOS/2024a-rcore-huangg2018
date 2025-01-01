//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::MAX_SYSCALL_NUM, mm::translated_byte_buffer, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_info, suspend_current_and_run_next, sysmmap, TaskStatus
    }, timer::get_time_us
};

#[repr(C)]
#[derive(Debug,Default)]
/// time valuses the current
pub struct TimeVal {
    /// seconds
    pub sec: usize,
    /// microseconds
    pub usec: usize,
}

impl TimeVal {
    fn new() -> Self {
        Self::default()
    }
}
/// get kernel system time value
pub fn get_kernel_system_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}
/// get kernel time
pub fn get_kernel_time() -> isize {
    let mut time = TimeVal::new();
    match get_kernel_system_time(&mut time, 0) {
        0 => ((time.sec & 0xffff) * 1000 + time.usec / 1000) as isize,
        _ => -1,
    }
}

/// Task information
#[allow(dead_code)]
#[derive(Clone, Copy,Debug)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}
impl TaskInfo {
    /// initialize task information
    pub fn new() -> Self {
        Self {
            status:TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
    /// set current task status
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
    /// record task call times
    pub fn add_syscall_time(&mut self, syscall_num: usize) {
        if syscall_num < MAX_SYSCALL_NUM {
            self.syscall_times[syscall_num] += 1;
        }
    }
    /// record task running time
    pub fn set_time(&mut self, time: usize) {
        self.time = time;
    }
    /// print syscall times
    pub fn print_syscall_times(&mut self) {
        for (indx, times) in self.syscall_times.iter().enumerate() {
            if *times!=0 {
                println!("syscall {}: {}", indx, times);
            }
        }
    }
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
    let buffers = translated_byte_buffer(
        current_user_token(), _ts as *const u8, size_of::<TimeVal>());

    let us = get_time_us();

    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let mut time_val_ptr = &time_val as *const _ as *const u8;
    for buffer in buffers {
        unsafe {
            time_val_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            time_val_ptr = time_val_ptr.add(buffer.len());
        }
    }
    // let us = get_time_us();
    // unsafe {
    //     *_ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let ti = get_current_task_info();
    //println!("ti: {:?}", ti);
    let buffers = translated_byte_buffer(
        current_user_token(), _ti as *const u8, size_of::<TaskInfo>());
    let mut ti_ptr = &ti as *const _ as *const u8;
    for buffer in buffers {
        unsafe {
            ti_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
            ti_ptr = ti_ptr.add(buffer.len());
        }
    }    
    0
}

// YOUR JOB: Implement mmap.
/// sys mmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port & !0x7 != 0 {
        println!("sys_mmap: start: {}, len: {}, port: {:03b}", _start, _len, _port as u8);
        sysmmap(_start, _len, _port+1);
    }
    0
}

/// sys munmap
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
