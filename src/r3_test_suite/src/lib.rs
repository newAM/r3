#![feature(external_doc)] // `#[doc(include = ...)]`
#![feature(const_fn)]
#![feature(const_panic)]
#![feature(const_option)]
#![feature(const_mut_refs)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(decl_macro)]
#![feature(is_sorted)]
#![feature(cell_update)]
#![feature(cfg_target_has_atomic)]
#![feature(array_windows)]
#![feature(unsafe_block_in_unsafe_fn)] // `unsafe fn` doesn't imply `unsafe {}`
#![deny(unsafe_op_in_unsafe_fn)]
#![doc(include = "./lib.md")]
#![doc(include = "./common.md")]
#![no_std]

mod utils;

// Generated by `build.rs`. Defines `get_selected_kernel_tests_inner!`.
include!(concat!(env!("OUT_DIR"), "/selective_tests.rs"));

/// Kernel tests
pub mod kernel_tests {
    use r3::kernel::{InterruptNum, InterruptPriority};
    /// Instantiation parameters of a test case.
    ///
    /// This trait has two purposes: (1) It serves as an interface to a test driver.
    /// It provides methods to notify the test driver of test success or failure.
    /// (2) It provides runtime access to the `App` structure.
    pub trait Driver<App> {
        /// Get a reference to `App` of the current test case.
        fn app() -> &'static App;

        /// Signal to the test runner that a test has succeeded.
        fn success();

        /// Signal to the test runner that a test has failed.
        fn fail();

        /// The list of interrupt lines that can be used by test programs.
        ///
        ///  - The list can have an arbitrary number of elements. Some tests
        ///    will be silently skipped if it's not enough. There should be at
        ///    least two for all test cases to run.
        ///
        ///  - There must be no duplicates.
        ///
        ///  - The port must support [`enable_interrupt_line`],
        ///    [`disable_interrupt_line`], [`pend_interrupt_line`] for all of
        ///    the specified interrupt lines.
        ///
        /// [`enable_interrupt_line`]: r3::kernel::PortInterrupts::enable_interrupt_line
        /// [`disable_interrupt_line`]: r3::kernel::PortInterrupts::disable_interrupt_line
        /// [`pend_interrupt_line`]: r3::kernel::PortInterrupts::pend_interrupt_line
        const INTERRUPT_LINES: &'static [InterruptNum] = &[];

        /// Valid managed priority values.
        ///
        ///  - The list can have an arbitrary number of elements. Some tests
        ///    will be silently skipped if it's not enough. There should be at
        ///    least two for all test cases to run.
        ///
        ///  - All elements must be in range
        ///    [`MANAGED_INTERRUPT_PRIORITY_RANGE`].
        ///
        ///  - The elements must be sorted in a descending order of priority.
        ///    That is, for every pair of adjacent elements `[p[i], p[i + 1]]`,
        ///    `p[i]` should be high enough to preempt `p[o + 1]`.
        ///
        /// [`MANAGED_INTERRUPT_PRIORITY_RANGE`]: r3::kernel::PortInterrupts::MANAGED_INTERRUPT_PRIORITY_RANGE
        const INTERRUPT_PRIORITIES: &'static [InterruptPriority] = &[];

        /// Valid unmanaged priority values.
        ///
        ///  - The list can have an arbitrary number of elements. Some tests
        ///    will be silently skipped if it's not enough. There should be at
        ///    least one for all test cases to run.
        ///
        ///  - No elements must be in range
        ///    [`MANAGED_INTERRUPT_PRIORITY_RANGE`].
        ///
        ///  - The elements must be sorted in a descending order of priority.
        ///    That is, for every pair of adjacent elements `[p[i], p[i + 1]]`,
        ///    `p[i]` should be high enough to preempt `p[o + 1]`.
        ///
        ///  - For every element `pri_unmanaged` in
        ///    `UNMANAGED_INTERRUPT_PRIORITIES` and every element `pri_managed`
        ///    in [`INTERRUPT_PRIORITIES`], `pri_unmanaged` should be high
        ///    enough to preempt `pri_managed`.
        ///
        /// [`MANAGED_INTERRUPT_PRIORITY_RANGE`]: r3::kernel::PortInterrupts::MANAGED_INTERRUPT_PRIORITY_RANGE
        /// [`INTERRUPT_PRIORITIES`]: Self::INTERRUPT_PRIORITIES
        ///
        const UNMANAGED_INTERRUPT_PRIORITIES: &'static [InterruptPriority] = &[];
    }

    macro_rules! define_kernel_tests {
        (
            [$dollar:tt] // get a `$` token
            $(
                // Test case definition
                $(#[cfg( $($cfg:tt)* )])*
                (mod $name_ident:ident {}, $name_str:literal)
            ),*$(,)*
        ) => {
            $(
                /// [**Test Case**]
                #[cfg(any(
                    feature = "tests_all",
                    all(feature = "tests_selective", kernel_test = $name_str)
                ))]
                #[cfg(all( $($( $cfg )*),* ))]
                pub mod $name_ident;

                /// This test case is not supported under the current feature set.
                #[cfg(any(
                    feature = "tests_all",
                    all(feature = "tests_selective", kernel_test = $name_str)
                ))]
                #[cfg(not(all( $($( $cfg )*),* )))]
                #[path = "disabled.rs"]
                pub mod $name_ident;
            )*

            /// The names of kernel tests.
            pub const TEST_NAMES: &[&str] = &[
                $( $name_str ),*
            ];

            /// Invoke the specified macro with a description of all defined
            /// kernel test cases.
            ///
            /// Note that the tests might not be actually compiled unless the
            /// feature `tests_all` is enabled.
            ///
            /// # Example
            ///
            /// ```rust,ignore
            /// r3_test_suite::get_kernel_tests!(aaa::bbb!(prefix));
            /// ```
            ///
            /// This expands to something like this:
            ///
            /// ```rust,ignore
            /// aaa::bbb!(
            ///     prefix
            ///     { path: r3_test_suite::test1, name_ident: test1, name_str: "test1", },
            ///     { path: r3_test_suite::test2, name_ident: test2, name_str: "test2", },
            /// );
            /// ```
            ///
            #[macro_export]
            macro_rules! get_kernel_tests {
                (
                    // Callback macro
                    $path:ident$dollar(::$path_sub:ident)* ! (
                        // Prefix of the callback parameter
                        $dollar($prefix:tt)*
                    )
                ) => {
                    $path$dollar($path_sub)* ! (
                        // Prefix
                        $dollar($prefix)*
                        $(
                            // The test info
                            {
                                path: $crate::kernel_tests::$name_ident,
                                name_ident: $name_ident,
                                name_str: $name_str,
                            },
                        )*
                    );
                };
            }
        };
    }

    define_kernel_tests! {
        [$]
        (mod basic {}, "basic"),
        (mod compute_atomics_round_robin {}, "compute_atomics_round_robin"),
        (mod compute_round_robin {}, "compute_round_robin"),
        (mod cpu_lock {}, "cpu_lock"),
        (mod event_group_interrupt {}, "event_group_interrupt"),
        (mod event_group_misc {}, "event_group_misc"),
        (mod event_group_order_fifo {}, "event_group_order_fifo"),
        (mod event_group_order_task_priority {}, "event_group_order_task_priority"),
        (mod event_group_poll {}, "event_group_poll"),
        (mod event_group_reorder_task_priority {}, "event_group_reorder_task_priority"),
        (mod event_group_set_and_dispatch {}, "event_group_set_and_dispatch"),
        (mod event_group_timeout {}, "event_group_timeout"),
        (mod event_group_wait_types {}, "event_group_wait_types"),
        (mod interrupt_disallowed_services {}, "interrupt_disallowed_services"),
        (mod interrupt_during_boot {}, "interrupt_during_boot"),
        (mod interrupt_handler_priority {}, "interrupt_handler_priority"),
        (mod interrupt_misc {}, "interrupt_misc"),
        (mod interrupt_priority {}, "interrupt_priority"),
        (mod interrupt_task_activate {}, "interrupt_task_activate"),
        (mod interrupt_unmanaged {}, "interrupt_unmanaged"),
        (mod mutex_misc {}, "mutex_misc"),
        (mod mutex_nesting {}, "mutex_nesting"),
        (mod mutex_protect_priority_by_ceiling {}, "mutex_protect_priority_by_ceiling"),
        (mod mutex_timeout {}, "mutex_timeout"),
        (mod mutex_unlock_and_dispatch {}, "mutex_unlock_and_dispatch"),
        (mod priority_boost {}, "priority_boost"),
        (mod semaphore_interrupt_handler {}, "semaphore_interrupt_handler"),
        (mod semaphore_misc {}, "semaphore_misc"),
        (mod semaphore_signal_and_dispatch {}, "semaphore_signal_and_dispatch"),
        (mod semaphore_timeout {}, "semaphore_timeout"),
        (mod startup_hook_disallowed_services {}, "startup_hook_disallowed_services"),
        (mod startup_hook_pend_interrupt {}, "startup_hook_pend_interrupt"),
        (mod startup_hook_priority {}, "startup_hook_priority"),
        (mod sync_mutex_lock_and_dispatch {}, "sync_mutex_lock_and_dispatch"),
        (mod sync_mutex_misc {}, "sync_mutex_misc"),
        (mod sync_recursive_mutex_lock_and_dispatch {}, "sync_recursive_mutex_lock_and_dispatch"),
        (mod sync_recursive_mutex_misc {}, "sync_recursive_mutex_misc"),
        (mod task_activate_and_dispatch {}, "task_activate_and_dispatch"),
        (mod task_activate_and_do_not_dispatch {}, "task_activate_and_do_not_dispatch"),
        (mod task_cpu_lock_reset {}, "task_cpu_lock_reset"),
        (mod task_misc {}, "task_misc"),
        (mod task_park {}, "task_park"),
        #[cfg(feature = "priority_boost")]
        (mod task_park_priority_boost {}, "task_park_priority_boost"),
        (mod task_park_reset {}, "task_park_reset"),
        (mod task_priority_boost_reset {}, "task_priority_boost_reset"),
        (mod task_priority_reset {}, "task_priority_reset"),
        (mod task_queue_fifo {}, "task_queue_fifo"),
        (mod task_set_priority {}, "task_set_priority"),
        (mod task_take_interrupt_at_return {}, "task_take_interrupt_at_return"),
        (mod time_adjust_event {}, "time_adjust_event"),
        #[cfg(feature = "priority_boost")]
        (mod time_adjust_limits {}, "time_adjust_limits"),
        (mod time_misc {}, "time_misc"),
        (mod time_set_event {}, "time_set_event"),
        #[cfg(feature = "system_time")]
        (mod time_stress {}, "time_stress"),
        (mod timer_misc {}, "timer_misc"),
        (mod timer_overdue {}, "timer_overdue"),
        (mod timer_periodic {}, "timer_periodic"),
        (mod timer_stop {}, "timer_stop"),
        (mod timer_zero_period {}, "timer_zero_period"),
    }

    /// Invoke the specified macro with a description of test cases
    /// selected by `R3_TEST`.
    ///
    /// Note that the tests might not be actually compiled unless the
    /// feature `tests_selective` is enabled.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// r3_test_suite::get_selected_kernel_tests!(aaa::bbb!(prefix));
    /// ```
    ///
    /// If there's an environment variable `R3_TEST=kernel_test::test1`,
    /// this expands to:
    ///
    /// ```rust,ignore
    /// aaa::bbb!(
    ///     prefix
    ///     { path: r3_test_suite::test1, name_ident: test1, name_str: "test1", },
    /// );
    /// ```
    ///
    #[macro_export]
    macro_rules! get_selected_kernel_tests {
        (
            // Callback macro
            $path:ident$(::$path_sub:ident)* ! (
                // Prefix of the callback parameter
                $($prefix:tt)*
            )
        ) => {
            // Forward to `get_selected_kernel_tests_inner!`
            $crate::get_selected_kernel_tests_inner!(
                ( $path $(::$path_sub)* ), ( $($prefix)* )
            );
        };
    }
}

/// Kernel benchmarks
pub mod kernel_benchmarks {
    use r3::kernel::{InterruptNum, InterruptPriority};

    /// Instantiation parameters of a test case.
    ///
    /// This trait has two purposes: (1) It serves as an interface to a test driver.
    /// It provides methods to notify the test driver of test success or failure.
    /// (2) It provides runtime access to the `App` structure.
    pub trait Driver<App> {
        /// Get a reference to `App` of the current test case.
        fn app() -> &'static App;

        /// Signal to the test runner that a test has succeeded.
        fn success();

        /// Get the current time for performance measurement.
        fn performance_time() -> u32;

        /// Get the unit of [`performance_time`] to be displayed following a
        /// measured value.
        ///
        /// [`performance_time`]: Self::performance_time
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// impl<A> r3_test_suite::kernel_benchmarks::Driver<A> for Driver {
        ///     /* ... */
        ///     fn performance_time() -> u32 {
        ///         winapi::um::sysinfoapi::GetTickCount()
        ///     }
        ///     const PERFORMANCE_TIME_UNIT: &'static str = "ms";
        /// }
        /// ```
        const PERFORMANCE_TIME_UNIT: &'static str;

        /// The list of interrupt lines that can be used by test programs.
        ///
        ///  - The list can have an arbitrary number of elements. Some tests
        ///    will be silently skipped if it's not enough. There should be at
        ///    least two for all test cases to run.
        ///
        ///  - There must be no duplicates.
        ///
        ///  - The port must support [`enable_interrupt_line`],
        ///    [`disable_interrupt_line`], [`pend_interrupt_line`] for all of
        ///    the specified interrupt lines.
        ///
        /// [`enable_interrupt_line`]: r3::kernel::PortInterrupts::enable_interrupt_line
        /// [`disable_interrupt_line`]: r3::kernel::PortInterrupts::disable_interrupt_line
        /// [`pend_interrupt_line`]: r3::kernel::PortInterrupts::pend_interrupt_line
        const INTERRUPT_LINES: &'static [InterruptNum] = &[];

        /// Valid priority values.
        ///
        ///  - The list can have an arbitrary number of elements. Some tests
        ///    will be silently skipped if it's not enough. There should be at
        ///    least two for all test cases to run.
        ///
        ///  - All elements must be in range
        ///    [`MANAGED_INTERRUPT_PRIORITY_RANGE`].
        ///
        ///  - The elements must be sorted in a descending order of priority.
        ///    That is, for every pair of adjacent elements `[p[i], p[i + 1]]`,
        ///    `p[i]` should be high enough to preempt `p[o + 1]`.
        ///
        /// [`MANAGED_INTERRUPT_PRIORITY_RANGE`]: r3::kernel::PortInterrupts::MANAGED_INTERRUPT_PRIORITY_RANGE
        const INTERRUPT_PRIORITIES: &'static [InterruptPriority] = &[];
    }

    /// The interface provided by [`use_benchmark_in_kernel_benchmark!`] and
    /// consumed by an inner app.
    trait Bencher<System, App> {
        /// Get a reference to `App` of the inner app.
        fn app() -> &'static App;

        fn mark_start();
        fn mark_end(int: crate::utils::benchmark::Interval);

        fn main_task() -> r3::kernel::Task<System>;
    }

    /// Define an `App` type using [the benchmark
    /// framework](crate::utils::benchmark).
    ///
    /// `struct $inner_ty<System>` should be defined in the same module.
    /// `$inner_ty<System>` should have the following methods:
    ///
    ///  - `const fn new<B: crate::kernel_benchmarks::Bencher<Self>>(b: &mut
    ///    CfgBuilder<System>) -> Self`
    ///  - `fn iter<B: crate::kernel_benchmarks::Bencher<Self>>()`
    ///
    /// # Safety
    ///
    /// See [`crate::utils::benchmark::BencherOptions`].
    ///
    macro_rules! use_benchmark_in_kernel_benchmark {
        {
            pub unsafe struct App<System> {
                inner: $inner_ty:ty,
            }
        } => {
            use crate::kernel_benchmarks::Driver;
            use crate::utils::benchmark;

            pub struct App<System> {
                benchmark: benchmark::BencherCottage<System>,
                inner: $inner_ty,
            }

            struct MyBencherOptions<System, D>(core::marker::PhantomData<(System, D)>);

            impl<System: r3::kernel::Kernel> App<System> {
                pub const fn new<D: Driver<Self>>(
                    b: &mut r3::kernel::cfg::CfgBuilder<System>,
                ) -> Self {
                    App {
                        benchmark: benchmark::configure::<System, MyBencherOptions<System, D>>(b),
                        inner: <$inner_ty>::new::<MyBencherOptions<System, D>>(b),
                    }
                }
            }

            /// benchmark framework → app
            unsafe impl<System: r3::kernel::Kernel, D: Driver<App<System>>>
                benchmark::BencherOptions<System> for MyBencherOptions<System, D>
            {
                fn cottage() -> &'static benchmark::BencherCottage<System> {
                    &D::app().benchmark
                }

                fn iter() {
                    <$inner_ty>::iter::<MyBencherOptions<System, D>>();
                }

                fn performance_time() -> u32 {
                    D::performance_time()
                }

                const PERFORMANCE_TIME_UNIT: &'static str = D::PERFORMANCE_TIME_UNIT;

                fn finish() {
                    D::success()
                }
            }

            /// app → benchmark framework
            impl<System: r3::kernel::Kernel, D: Driver<App<System>>>
                crate::kernel_benchmarks::Bencher<System, $inner_ty> for MyBencherOptions<System, D>
            {
                fn app() -> &'static $inner_ty {
                    &D::app().inner
                }

                fn mark_start() {
                    <Self as benchmark::Bencher<System>>::mark_start()
                }
                fn mark_end(int: benchmark::Interval) {
                    <Self as benchmark::Bencher<System>>::mark_end(int)
                }

                fn main_task() -> r3::kernel::Task<System> {
                    <Self as benchmark::Bencher<System>>::main_task()
                }
            }
        };
    }

    macro_rules! define_kernel_benchmarks {
        (
            [$dollar:tt] // get a `$` token
            $(
                // Test case definition
                (mod $name_ident:ident {}, $name_str:literal)
            ),*$(,)*
        ) => {
            $(
                /// [**Test Case**]
                #[cfg(any(
                    feature = "tests_all",
                    all(feature = "tests_selective", kernel_benchmark = $name_str)
                ))]
                pub mod $name_ident;
            )*

            /// The names of kernel benchmark tests.
            pub const TEST_NAMES: &[&str] = &[
                $( $name_str ),*
            ];

            /// Invoke the specified macro with a description of all defined
            /// kernel benchmark test cases.
            ///
            /// Note that the tests might not be actually compiled unless the
            /// feature `tests_all` is enabled.
            ///
            /// # Example
            ///
            /// ```rust,ignore
            /// r3_test_suite::get_kernel_benchmarks!(aaa::bbb!(prefix));
            /// ```
            ///
            /// This expands to something like this:
            ///
            /// ```rust,ignore
            /// aaa::bbb!(
            ///     prefix
            ///     { path: r3_test_suite::test1, name_ident: test1, name_str: "test1", },
            ///     { path: r3_test_suite::test2, name_ident: test2, name_str: "test2", },
            /// );
            /// ```
            ///
            #[macro_export]
            macro_rules! get_kernel_benchmarks {
                (
                    // Callback macro
                    $path:ident$dollar(::$path_sub:ident)* ! (
                        // Prefix of the callback parameter
                        $dollar($prefix:tt)*
                    )
                ) => {
                    $path$dollar($path_sub)* ! (
                        // Prefix
                        $dollar($prefix)*
                        $(
                            // The test info
                            {
                                path: $crate::kernel_benchmarks::$name_ident,
                                name_ident: $name_ident,
                                name_str: $name_str,
                            },
                        )*
                    );
                };
            }
        };
    }

    define_kernel_benchmarks! {
        [$]
        (mod mutex_ceiling {}, "mutex_ceiling"),
        (mod mutex_none {}, "mutex_none"),
        (mod port {}, "port"),
        (mod semaphore {}, "semaphore"),
        (mod task_lifecycle {}, "task_lifecycle"),
    }

    #[cfg(any(
        feature = "tests_all",
        all(feature = "tests_selective", kernel_benchmark = "mutex_none"),
        all(feature = "tests_selective", kernel_benchmark = "mutex_ceiling"),
    ))]
    mod mutex;

    /// Invoke the specified macro with a description of test cases
    /// selected by `R3_TEST`.
    ///
    /// Note that the tests might not be actually compiled unless the
    /// feature `tests_selective` is enabled.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// r3_test_suite::get_selected_kernel_benchmarks!(aaa::bbb!(prefix));
    /// ```
    ///
    /// If there's an environment variable `R3_TEST=kernel_benchmark::test1`,
    /// this expands to:
    ///
    /// ```rust,ignore
    /// aaa::bbb!(
    ///     prefix
    ///     { path: r3_test_suite::test1, name_ident: test1, name_str: "test1", },
    /// );
    /// ```
    ///
    #[macro_export]
    macro_rules! get_selected_kernel_benchmarks {
        (
            // Callback macro
            $path:ident$(::$path_sub:ident)* ! (
                // Prefix of the callback parameter
                $($prefix:tt)*
            )
        ) => {
            // Forward to `get_selected_kernel_benchmarks_inner!`
            $crate::get_selected_kernel_benchmarks_inner!(
                ( $path $(::$path_sub)* ), ( $($prefix)* )
            );
        };
    }
}
