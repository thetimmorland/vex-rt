use libc::{c_char, c_void};

pub type Task = *mut c_void;

#[repr(C)]
pub enum TaskState {
    Running,
    Ready,
    Blocked,
    Suspended,
    Deleted,
    Invalid,
}

#[repr(C)]
pub enum NotifyAction {
    None,
    Bits,
    Incr,
    OWrite,
    NoOWrite,
}

extern "C" {
    pub fn millis() -> u32;
    pub fn task_create(
        callback: extern "C" fn(Task),
        parameters: Task,
        prio: u32,
        stack_depth: u16,
        name: *const c_char,
    ) -> Task;
    pub fn task_delete(task: Task);
    pub fn task_delay(milliseconds: u32);
    pub fn task_delay_until(prev_time: *mut u32, delta: u32);
    pub fn task_get_priority(task: Task) -> u32;
    pub fn task_set_priority(task: Task, prio: u32) -> u32;
    pub fn task_get_state(task: Task) -> TaskState;
    pub fn task_suspend(task: Task);
    pub fn task_resume(task: Task);
    pub fn task_get_count() -> u32;
    pub fn task_get_name(task: Task) -> *const c_char;
    pub fn task_get_by_name(name: *const c_char) -> Task;
    pub fn task_get_current() -> Task;
    pub fn task_notify(task: Task);
    pub fn task_notify_ext(
        task: Task,
        value: u32,
        action: NotifyAction,
        prev_value: *mut u32,
    ) -> u32;
    pub fn task_notify_take(clear_on_exit: bool, timeout: u32) -> u32;
    pub fn task_notify_clear(task: Task) -> bool;
}
