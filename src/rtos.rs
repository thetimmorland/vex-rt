use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use core::fmt::{Debug, Display, Formatter};
use core::{cell::UnsafeCell, fmt};
use core::{ops::Deref, time::Duration};
use core::{ops::DerefMut, ptr::null_mut};

use crate::{
    bindings,
    error::*,
    util::{
        owner::Owner,
        shared_set::{insert, SharedSet, SharedSetHandle},
        *,
    },
};

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

    /// Delays the current task by the specified duration.
    pub fn delay(dur: Duration) {
        unsafe {
            bindings::task_delay(dur.as_millis() as u32);
        }
    }

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

    /// Gets the name of the task.
    pub fn name(&self) -> String {
        unsafe { from_cstring_raw(bindings::task_get_name(self.0)) }
    }

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

/// Represents an object which is protected by a FreeRTOS mutex.
pub struct Mutex<T: ?Sized> {
    mutex: bindings::mutex_t,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

unsafe impl<T: ?Sized + Sync> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new mutex which wraps the given object. Panics on failure; see
    /// [`Mutex::try_new()`].
    pub fn new(data: T) -> Self {
        Self::try_new(data).unwrap_or_else(|err| panic!("failed to create mutex: {:?}", err))
    }

    /// Creates a new mutex which wraps the given object.
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
    /// Obtains a [`MutexGuard`] giving access to the object protected by the
    /// mutex. Blocks until access can be obtained. Panics on failure; see
    /// [`Mutex::try_lock()`].
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        self.try_lock()
            .unwrap_or_else(|err| panic!("Failed to lock mutex: {:?}", err))
    }

    /// Obtains a [`MutexGuard`] giving access to the object protected by the
    /// mutex. Blocks until access can be obtained.
    pub fn try_lock<'a>(&'a self) -> Result<MutexGuard<'a, T>, Error> {
        if unsafe { bindings::mutex_take(self.mutex, TIMEOUT_MAX) } {
            Ok(MutexGuard(self))
        } else {
            Err(from_errno())
        }
    }

    /// Obtains a [`MutexGuard`] giving access to the object protected by the
    /// mutex, if it is available immediately. Does not block.
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

/// Provides exclusive access to an object controlled by a [`Mutex`] via the
/// RAII pattern.
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

/// Provides a constant-period looping construct.
pub struct Loop {
    last_time: u32,
    delta: u32,
}

impl Loop {
    /// Creates a new loop object with a given period.
    pub fn new(delta: Duration) -> Loop {
        Loop {
            last_time: unsafe { bindings::millis() },
            delta: delta.as_millis() as u32,
        }
    }

    /// Delays until the next loop cycle.
    pub fn delay(&mut self) {
        unsafe { bindings::task_delay_until(&mut self.last_time, self.delta) }
    }

    /// A [`Selectable`] event which occurs at the next loop cycle.
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

#[macro_export]
#[doc(hidden)]
macro_rules! select_head {
    ($event:expr,) => {$event};
    ($event:expr, $($rest:expr,)+) => {($event, select_head!($($rest,)*))}
}

#[macro_export]
#[doc(hidden)]
macro_rules! select_match {
    { $event:expr; $cons:expr; $_:expr, } => {
        match $crate::Selectable::poll($event) {
            ::core::result::Result::Ok(r) => break $cons(r),
            ::core::result::Result::Err(s) => s,
        }
    };
    { $events:expr; $cons:expr; $_:expr, $($rest:expr,)+ } => {
        match $crate::Selectable::poll($events.0) {
            ::core::result::Result::Ok(r) => break $cons(::core::result::Result::Ok(r)),
            ::core::result::Result::Err(s) => (s, select_match!{$events.1; |r| $cons(::core::result::Result::Err(r)); $($rest,)*}),
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! select_body {
    { $result:expr; $var:pat => $body:expr, } => {
        match $result {
            $var => $body,
        }
    };
    { $result:expr; $var:pat => $body:expr, $($vars:pat => $bodys:expr,)+ } => {
        match $result {
            ::core::result::Result::Ok($var) => $body,
            ::core::result::Result::Err(r) => select_body!{r; $($vars => $bodys,)*},
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! select_sleep {
    ($events:expr; $_:expr,) => {$events.sleep()};
    ($events:expr; $_:expr, $($rest:expr,)+) => {$events.0.sleep().combine(select_sleep!($events.1; $($rest,)+))};
}

#[macro_export]
/// Selects over a range of possible future events, processing exactly one.
/// Inspired by equivalent behaviours in other programming languages such as Go
/// and Kotlin, and ultimately the `select` system call from POSIX.
///
/// Which event gets processed is a case of bounded non-determinism: the
/// implementation makes no guarantee about which event gets processed if
/// multiple become possible around the same time, only that it will process one
/// of them if at least one can be processed.
///
/// # Examples
///
/// ```
/// fn foo(ctx: Context) {
///     let mut x = 0;
///     let mut l = Loop::new(Duration::from_secs(1));
///     loop {
///         println!("x = {}", x);
///         x += 1;
///         select! {
///             _ = l.next() => continue,
///             _ = ctx.done() => break,
///         }
///     }
/// }
/// ```
macro_rules! select {
    { $( $var:pat = $event:expr => $body:expr ),+ $(,)? } => {{
        let mut events = $crate::select_head!($($event,)+);
        select_body!{loop {
            events = $crate::select_match!{events; |r| r; $($event,)+};
            $crate::select_sleep!(events; $($event,)+).sleep();
        }; $($var => $body,)+}
    }};
}

/// Represents a self-maintaining set of tasks to notify when an event occurs.
pub struct Event(SharedSet<Task>);

impl Event {
    /// Creates a new event structure with an empty set of tasks.
    pub fn new() -> Self {
        Event(SharedSet::new())
    }

    /// Notify the tasks which are waiting for an event.
    pub fn notify(&self) {
        for t in self.0.iter() {
            unsafe { bindings::task_notify(t.0) };
        }
    }
}

/// Represents a handle into the listing of the current task in an [`Event`].
/// When this handle is dropped, that task is removed from the event's set.
pub struct EventHandle<O: Owner<Event>>(Option<SharedSetHandle<Task, EventHandleOwner<O>>>);

struct EventHandleOwner<O: Owner<Event>>(O);

impl<O: Owner<Event>> Owner<SharedSet<Task>> for EventHandleOwner<O> {
    fn with<U>(&self, f: impl FnOnce(&mut SharedSet<Task>) -> U) -> Option<U> {
        self.0.with(|e| f(&mut e.0))
    }
}

/// Adds the current task to the notification set for an [`Event`], acquiring an
/// [`EventHandle`] to manage the lifetime of that entry.
pub fn handle_event<O: Owner<Event>>(owner: O) -> EventHandle<O> {
    EventHandle(insert(EventHandleOwner(owner), Task::current()))
}
