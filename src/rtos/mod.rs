//! Multitasking primitives.

use alloc::{boxed::Box, format, string::String};
use core::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    ptr::null_mut,
    time::Duration,
};

use crate::{bindings, error::*, util::*};

const TIMEOUT_MAX: u32 = 0xffffffff;

/// Gets the current timestamp (i.e., the time which has passed since program
/// start).
pub fn time_since_start() -> Duration {
    unsafe { Duration::from_millis(bindings::millis().into()) }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Represents a FreeRTOS task.
pub struct Task(bindings::task_t);

impl Task {
    /// The default priority for new tasks.
    pub const DEFAULT_PRIORITY: u32 = bindings::TASK_PRIORITY_DEFAULT;

    /// The default stack depth for new tasks.
    pub const DEFAULT_STACK_DEPTH: u16 = bindings::TASK_STACK_DEPTH_DEFAULT as u16;

    #[inline]
    /// Delays the current task by the specified duration.
    pub fn delay(dur: Duration) {
        unsafe {
            bindings::task_delay(dur.as_millis() as u32);
        }
    }

    #[inline]
    /// Gets the current task.
    pub fn current() -> Task {
        Task(unsafe { bindings::task_get_current() })
    }

    /// Finds a task by its name.
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

    #[inline]
    /// Spawns a new task with no name and the default priority and stack depth.
    pub fn spawn<F>(f: F) -> Result<Task, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        Task::spawn_ext("", Self::DEFAULT_PRIORITY, Self::DEFAULT_STACK_DEPTH, f)
    }

    /// Spawns a new task with the specified name, priority and stack depth.
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

    #[inline]
    /// Spawns a new task from a C function pointer and an arbitrary data
    /// pointer.
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

    #[inline]
    /// Gets the name of the task.
    pub fn name(&self) -> String {
        unsafe { from_cstring_raw(bindings::task_get_name(self.0)) }
    }

    #[inline]
    /// Gets the priority of the task.
    pub fn priority(&self) -> u32 {
        unsafe { bindings::task_get_priority(self.0) }
    }
}

impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("name", &self.name())
            .field("priority", &self.priority())
            .finish()
    }
}

unsafe impl Send for Task {}

unsafe impl Sync for Task {}

#[derive(Copy, Clone, Debug)]
/// Represents a future time to sleep until.
pub enum GenericSleep {
    /// Represents a future time when a notification occurs. If a timestamp is
    /// present, then it represents whichever is earlier.
    NotifyTake(Option<Duration>),
    /// Represents an explicit future timestamp.
    Timestamp(Duration),
}

impl GenericSleep {
    /// Sleeps until the future time respresented by `self`. The result is the
    /// number of notifications which were present, if the sleep ended due to
    /// notification.
    pub fn sleep(self) -> u32 {
        match self {
            GenericSleep::NotifyTake(timeout) => {
                let timeout = timeout.map_or(TIMEOUT_MAX, |v| {
                    v.checked_sub(time_since_start())
                        .map_or(0, |d| d.as_millis() as u32)
                });
                unsafe { bindings::task_notify_take(true, timeout) }
            }
            GenericSleep::Timestamp(v) => {
                if let Some(d) = v.checked_sub(time_since_start()) {
                    Task::delay(d)
                }
                0
            }
        }
    }

    #[inline]
    /// Get the timestamp represented by `self`, if it is present.
    pub fn timeout(self) -> Option<Duration> {
        match self {
            GenericSleep::NotifyTake(v) => v,
            GenericSleep::Timestamp(v) => Some(v),
        }
    }

    /// Combine two `GenericSleep` objects to one which represents the earliest
    /// possible time of the two.
    pub fn combine(self, other: Self) -> Self {
        match (self, other) {
            (GenericSleep::Timestamp(a), GenericSleep::Timestamp(b)) => {
                GenericSleep::Timestamp(core::cmp::min(a, b))
            }
            (a, b) => GenericSleep::NotifyTake(a.timeout().map_or(b.timeout(), |a| {
                Some(b.timeout().map_or(a, |b| core::cmp::min(a, b)))
            })),
        }
    }
}

/// Represents a future event which can be used with the [`select!`] macro.
pub trait Selectable<T = ()>: Sized {
    /// Processes the event if it is ready, consuming the event object;
    /// otherwise, it provides a replacement event object.
    fn poll(self) -> Result<T, Self>;
    /// Gets the earliest time that the event could be ready.
    fn sleep(&self) -> GenericSleep;
}

/// Creates a new [`Selectable`] event by mapping the result of a given one.
#[inline]
pub fn select_map<'a, T: 'a, U: 'a, F: 'a + FnOnce(T) -> U>(
    event: impl Selectable<T> + 'a,
    f: F,
) -> impl Selectable<U> + 'a {
    struct MapSelect<T, U, E: Selectable<T>, F: FnOnce(T) -> U> {
        event: E,
        f: F,
        _t: PhantomData<T>,
    }

    impl<T, U, E: Selectable<T>, F: FnOnce(T) -> U> Selectable<U> for MapSelect<T, U, E, F> {
        fn poll(self) -> Result<U, Self> {
            match self.event.poll() {
                Ok(r) => Ok((self.f)(r)),
                Err(event) => Err(Self {
                    event,
                    f: self.f,
                    _t: PhantomData,
                }),
            }
        }
        fn sleep(&self) -> GenericSleep {
            self.event.sleep()
        }
    }

    MapSelect {
        event,
        f,
        _t: PhantomData,
    }
}

/// Creates a new [`Selectable`] event which processes exactly one of the given
/// events.
#[inline]
pub fn select_either<'a, T: 'a>(
    fst: impl Selectable<T> + 'a,
    snd: impl Selectable<T> + 'a,
) -> impl Selectable<T> + 'a {
    struct EitherSelect<T, E1: Selectable<T>, E2: Selectable<T>>(E1, E2, PhantomData<T>);

    impl<T, E1: Selectable<T>, E2: Selectable<T>> Selectable<T> for EitherSelect<T, E1, E2> {
        fn poll(self) -> Result<T, Self> {
            Err(Self(
                match self.0.poll() {
                    Ok(r) => return Ok(r),
                    Err(e) => e,
                },
                match self.1.poll() {
                    Ok(r) => return Ok(r),
                    Err(e) => e,
                },
                PhantomData,
            ))
        }
        fn sleep(&self) -> GenericSleep {
            self.0.sleep().combine(self.1.sleep())
        }
    }

    EitherSelect(fst, snd, PhantomData)
}

mod context;
mod event;
mod r#loop;
mod mutex;

pub use context::*;
pub use event::*;
pub use mutex::*;
pub use r#loop::*;
