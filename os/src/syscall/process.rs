//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,get_current_task_info},
    timer::get_time_us,
};
/// time duration
#[repr(C)]
#[derive(Debug,Default)]
pub struct TimeVal {
    /// seconds
    pub sec: usize,
    /// microseconds
    pub usec: usize,
}
impl TimeVal {
    fn new()->Self {
        Self::default()
    }
}
/// Task information
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

impl TaskInfo {
    /// Create a new TaskInfo with uninitialized status and syscall times
    pub fn new()->Self {
        TaskInfo{
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
    /// Increase syscall count by 1
    pub fn inc_syscall_times(&mut self, syscall_id: usize) {
        if syscall_id < MAX_SYSCALL_NUM {
            self.syscall_times[syscall_id] += 1;
        }
    }

    /// calculating running time
    pub fn set_spend_time(&mut self, duration: usize) {
        //println!("spending time: {}", duration);
        self.time = duration;
    }

    ///
    pub fn set_task_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
    /// print task syscall times
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}
/// get time in kernel
pub fn get_time_kernel() -> isize {
    let mut t = TimeVal::new();
    match sys_get_time(&mut t, 0) {
        0 => ((t.sec & 0xffff) * 1000 + t.usec / 1000) as isize,
        _ => -1,
    }
}
/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let ti = get_current_task_info();
    trace!("kernel: sys_task_info:spend time:{:?}",ti.time);
    unsafe {
    *_ti = ti;
    }
    0
}
