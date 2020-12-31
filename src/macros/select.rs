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
        $crate::select_body!{loop {
            $crate::rtos::GenericSleep::sleep($crate::select_sleep!(events; $($event,)+));
            events = $crate::select_match!{events; |r| r; $($event,)+};
        }; $($var => $body,)+}
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! select_head {
    ($event:expr,) => {$event};
    ($event:expr, $($rest:expr,)+) => {($event, $crate::select_head!($($rest,)*))}
}

#[macro_export]
#[doc(hidden)]
macro_rules! select_match {
    { $event:expr; $cons:expr; $_:expr, } => {
        match $crate::rtos::Selectable::poll($event) {
            ::core::result::Result::Ok(r) => break $cons(r),
            ::core::result::Result::Err(s) => s,
        }
    };
    { $events:expr; $cons:expr; $_:expr, $($rest:expr,)+ } => {
        match $crate::rtos::Selectable::poll($events.0) {
            ::core::result::Result::Ok(r) => break $cons(::core::result::Result::Ok(r)),
            ::core::result::Result::Err(s) => (
                s,
                $crate::select_match!{$events.1; |r| $cons(::core::result::Result::Err(r)); $($rest,)*}
            ),
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
            ::core::result::Result::Err(r) => $crate::select_body!{r; $($vars => $bodys,)*},
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! select_sleep {
    ($event:expr; $_:expr,) => {$crate::rtos::Selectable::sleep(&$event)};
    ($events:expr; $_:expr, $($rest:expr,)+) => {
        $crate::rtos::Selectable::sleep(&$events.0).combine($crate::select_sleep!($events.1; $($rest,)+))
    };
}
