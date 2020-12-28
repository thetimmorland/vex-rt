use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use core::fmt::{Debug, Display, Formatter};
use core::{cell::UnsafeCell, fmt};
use core::{ops::Deref, time::Duration};
use core::{ops::DerefMut, ptr::null_mut};

use crate::bindings;
use crate::error::*;
use crate::util::*;

const TIMEOUT_MAX: u32 = 0xffffffff;

pub fn time_since_start() -> Duration {
    unsafe { Duration::from_millis(bindings::millis().into()) }
}

pub struct Task(bindings::task_t);

impl Task {
    pub const DEFAULT_PRIORITY: u32 = bindings::TASK_PRIORITY_DEFAULT;
    pub const DEFAULT_STACK_DEPTH: u16 = bindings::TASK_STACK_DEPTH_DEFAULT as u16;

    pub fn delay(dur: Duration) {
        unsafe {
            bindings::task_delay(dur.as_millis() as u32);
        }
    }

    pub fn current() -> Task {
        Task(unsafe { bindings::task_get_current() })
    }

    pub fn find_by_name(name: &str) -> Result<Task, Error> {
        let ptr = as_cstring(name, |cname| unsafe {
            Ok(bindings::task_get_by_name(cname.into_raw()))
        })?;
        if ptr == null_mut() {
            Err(Error::Custom(format!("task not found: {}", name)))
        } else {
            Ok(Task(ptr))
        }
    }

    pub fn spawn<F>(f: F) -> Result<Task, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        Task::spawn_ext("", Self::DEFAULT_PRIORITY, Self::DEFAULT_STACK_DEPTH, f)
    }

    pub fn spawn_ext<F>(name: &str, priority: u32, stack_depth: u16, f: F) -> Result<Task, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        extern "C" fn run<F: FnOnce()>(arg: *mut libc::c_void) {
            let cb_box: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            cb_box()
        }

        let cb = Box::new(f);
        unsafe {
            let arg = Box::into_raw(cb);
            let r = Task::spawn_raw(name, priority, stack_depth, run::<F>, arg as *mut _);
            if let Err(_) = r {
                // We need to re-box the pointer if the task could not be created, to avoid a
                // memory leak.
                Box::from_raw(arg);
            }
            r
        }
    }

    pub unsafe fn spawn_raw(
        name: &str,
        priority: u32,
        stack_depth: u16,
        f: unsafe extern "C" fn(arg1: *mut libc::c_void),
        arg: *mut libc::c_void,
    ) -> Result<Task, Error> {
        as_cstring(name, |cname| {
            let task = bindings::task_create(Some(f), arg, priority, stack_depth, cname.into_raw());
            if task != null_mut() {
                Ok(Task(task))
            } else {
                Err(from_errno())
            }
        })
    }

    pub fn name(&self) -> String {
        unsafe { from_cstring_raw(bindings::task_get_name(self.0)) }
    }
}

pub struct Mutex<T: ?Sized> {
    mutex: bindings::mutex_t,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Self::try_new(data).unwrap_or_else(|err| panic!("failed to create mutex: {:?}", err))
    }

    pub fn try_new(data: T) -> Result<Self, Error> {
        let mutex = unsafe { bindings::mutex_create() };
        if mutex != null_mut() {
            Ok(Self {
                data: UnsafeCell::new(data),
                mutex,
            })
        } else {
            Err(from_errno())
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        self.try_lock()
            .unwrap_or_else(|err| panic!("Failed to lock mutex: {:?}", err))
    }

    pub fn try_lock<'a>(&'a self) -> Result<MutexGuard<'a, T>, Error> {
        if unsafe { bindings::mutex_take(self.mutex, TIMEOUT_MAX) } {
            Ok(MutexGuard(self))
        } else {
            Err(from_errno())
        }
    }

    pub fn poll<'a>(&'a self) -> Option<MutexGuard<'a, T>> {
        if unsafe { bindings::mutex_take(self.mutex, 0) } {
            Some(MutexGuard(self))
        } else {
            None
        }
    }
}

impl<T: ?Sized> Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe { bindings::mutex_delete(self.mutex) }
    }
}

impl<T: ?Sized + Debug> Debug for Mutex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.poll() {
            Some(guard) => f.debug_struct("Mutex").field("data", &&*guard).finish(),
            None => {
                struct LockedPlaceholder;
                impl Debug for LockedPlaceholder {
                    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                        f.write_str("<locked>")
                    }
                }

                f.debug_struct("Mutex")
                    .field("data", &LockedPlaceholder)
                    .finish()
            }
        }
    }
}

impl<T: ?Sized + Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

pub struct MutexGuard<'a, T: ?Sized>(&'a Mutex<T>);

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        if !unsafe { bindings::mutex_give(self.0.mutex) } {
            panic!("failed to return mutex: {:?}", from_errno());
        }
    }
}

impl<T: ?Sized + Debug> Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + Display> Display for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> !Send for MutexGuard<'_, T> {}

unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

pub struct Loop {
    last_time: u32,
    delta: u32,
}

impl Loop {
    pub fn new(delta: Duration) -> Loop {
        Loop {
            last_time: unsafe { bindings::millis() },
            delta: delta.as_millis() as u32,
        }
    }

    pub fn delay(&mut self) {
        unsafe { bindings::task_delay_until(&mut self.last_time, self.delta) }
    }

    pub fn next<'a>(&'a mut self) -> impl Selectable + 'a {
        struct LoopSelect<'a>(&'a mut Loop);

        impl<'a> Selectable for LoopSelect<'a> {
            fn poll(self) -> Result<(), Self> {
                if unsafe { bindings::millis() } >= self.0.last_time + self.0.delta {
                    self.0.last_time += self.0.delta;
                    Ok(())
                } else {
                    Err(self)
                }
            }
            fn sleep(&self) -> GenericSleep {
                GenericSleep::Timestamp(Duration::from_millis(
                    (self.0.last_time + self.0.delta).into(),
                ))
            }
        }

        LoopSelect(self)
    }
}

#[derive(Copy, Clone)]
pub enum GenericSleep {
    NotifyTake(Option<Duration>),
    Timestamp(Duration),
}

impl GenericSleep {
    pub fn sleep(self) -> u32 {
        match self {
            GenericSleep::NotifyTake(timeout) => {
                let timeout =
                    timeout.map_or(TIMEOUT_MAX, |v| (time_since_start() - v).as_millis() as u32);
                unsafe { bindings::task_notify_take(true, timeout) }
            }
            GenericSleep::Timestamp(v) => {
                Task::delay(time_since_start() - v);
                0
            }
        }
    }

    pub fn timeout(self) -> Option<Duration> {
        match self {
            GenericSleep::NotifyTake(v) => v,
            GenericSleep::Timestamp(v) => Some(v),
        }
    }
}

impl core::ops::BitOr for GenericSleep {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (GenericSleep::Timestamp(a), GenericSleep::Timestamp(b)) => {
                GenericSleep::Timestamp(core::cmp::min(a, b))
            }
            (a, b) => GenericSleep::NotifyTake(a.timeout().map_or(b.timeout(), |a| {
                Some(b.timeout().map_or(a, |b| core::cmp::min(a, b)))
            })),
        }
    }
}

pub trait Selectable<T = ()> {
    fn poll(self) -> Result<T, Self>
    where
        Self: Sized;
    fn sleep(&self) -> GenericSleep;
}

#[macro_export]
macro_rules! select_head {
    () => {()};
    ($event:expr, $($rest:expr),*) => {($event, select_head!($($rest),*))}
}

#[macro_export]
macro_rules! select_body {
    { $events:expr; } => {$events};
    { $events:expr; $var:pat => $body:expr, $($vars:pat => $bodys:expr),* } => {
        match $crate::Selectable::poll($events.0) {
            ::core::result::Result::Ok($var) => break $body,
            ::core::result::Result::Err(s) => (s, select_body!{$events.1; $($vars => $bodys,)*}),
        }
    };
}

#[macro_export]
macro_rules! select_sleep {
    ($events:expr; $_:expr,) => {$events.0.sleep()};
    ($events:expr; $_:expr, $($rest:expr),+) => {$events.0.sleep() | select_sleep!($events.1; $($rest,)+)};
}

#[macro_export]
macro_rules! select {
    { $( $var:pat = $event:expr => $body:expr ),+ } => {{
        let mut events = $crate::select_head!($($event,)+);
        loop {
            events = $crate::select_body!{events; $($var => $body,)+};
            $crate::select_sleep!(events; $($event,)+).sleep();
        }
    }};
}
