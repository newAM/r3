#![feature(unsafe_block_in_unsafe_fn)] // `unsafe fn` doesn't imply `unsafe {}`
#![deny(unsafe_op_in_unsafe_fn)]
use atomic_ref::AtomicRef;
use constance::prelude::*;
use std::{
    sync::atomic::AtomicU8,
    thread::{self, JoinHandle},
};

#[doc(hidden)]
pub use constance::kernel::{Port, PortToKernel, TaskCb};
/// Used by `use_port!`
#[doc(hidden)]
pub use std::sync::atomic::{AtomicBool, Ordering};
#[doc(hidden)]
pub extern crate env_logger;

#[doc(hidden)]
pub struct State {
    cpu_lock: AtomicBool,
    dispatcher: AtomicRef<'static, thread::Thread>,
    dispatcher_pending: AtomicBool,
}

#[derive(Debug)]
pub struct TaskState {
    thread: AtomicRef<'static, JoinHandle<()>>,
    tsm: AtomicU8,
}

// Task state machine
const TSM_UNINIT: u8 = 0;
const TSM_DORMANT: u8 = 1;
const TSM_RUNNING: u8 = 2;
const TSM_RUNNABLE: u8 = 3;

impl Init for TaskState {
    const INIT: Self = Self::new();
}

impl TaskState {
    pub const fn new() -> Self {
        Self {
            thread: AtomicRef::new(None),
            tsm: AtomicU8::new(TSM_UNINIT),
        }
    }

    /// Yield the current task `self` and invoke the dispatcher.
    fn yield_current(&self, state: &State) {
        log::trace!("yield_current({:p}) enter", self);

        // `self` must represent the current thread
        assert_eq!(
            Some(thread::current().id()),
            self.thread
                .load(Ordering::Relaxed)
                .map(|jh| jh.thread().id()),
            "`self` is not a current thread"
        );

        self.tsm.store(TSM_RUNNABLE, Ordering::Release);

        // Unpark the dispatcher
        state.invoke_dispatcher();

        // Suspend the current thread until woken up
        while self.tsm.load(Ordering::Acquire) != TSM_RUNNING {
            thread::park();
        }
        log::trace!("yield_current({:p}) leave", self);
    }
}

impl State {
    pub const fn new() -> Self {
        Self {
            cpu_lock: AtomicBool::new(true),
            dispatcher: AtomicRef::new(None),
            dispatcher_pending: AtomicBool::new(true),
        }
    }

    fn invoke_dispatcher(&self) {
        let dispatcher = self.dispatcher.load(Ordering::Relaxed).unwrap();
        self.dispatcher_pending.store(true, Ordering::Release);
        dispatcher.unpark();
    }

    pub unsafe fn dispatch_first_task<System: Kernel>(&'static self) -> !
    where
        System: Port<PortTaskState = TaskState>,
    {
        log::trace!("dispatch_first_task");
        assert!(self.is_cpu_lock_active());

        // This thread becomes the dispatcher
        self.dispatcher.store(
            Some(Box::leak(Box::new(thread::current()))),
            Ordering::Relaxed,
        );

        loop {
            if !self.dispatcher_pending.swap(false, Ordering::Acquire) {
                thread::park();
                continue;
            }

            // Enable CPU Lock
            self.cpu_lock.store(true, Ordering::Relaxed);

            // Let the kernel decide the next task to run
            // Safety: CPU Lock enabled (implied by us being the dispatcher)
            unsafe {
                System::choose_running_task();
            }

            // Run that task
            if let Some(task) = System::state().running_task() {
                let pts = &task.port_task_state;
                assert_eq!(pts.tsm.load(Ordering::Relaxed), TSM_DORMANT);

                if pts.thread.load(Ordering::Relaxed).is_none() {
                    // Start the task's thread
                    let jh = thread::Builder::new()
                        .spawn(move || loop {
                            while pts.tsm.load(Ordering::Acquire) != TSM_RUNNING {
                                thread::park();
                            }

                            assert!(!self.is_cpu_lock_active());

                            log::debug!("task {:p} is now running", task);

                            // Safety: The port can call this
                            unsafe {
                                (task.attr.entry_point)(task.attr.entry_param);
                            }

                            // Sleep until woken up again
                            pts.tsm.store(TSM_UNINIT, Ordering::Relaxed);
                        })
                        .unwrap();
                    pts.thread
                        .store(Some(Box::leak(Box::new(jh))), Ordering::Relaxed);
                }

                // Disable CPU Lock
                self.cpu_lock.store(false, Ordering::Relaxed);

                // Unpark the task's thread
                pts.tsm.store(TSM_RUNNING, Ordering::Release);
                pts.thread
                    .load(Ordering::Relaxed)
                    .unwrap()
                    .thread()
                    .unpark();
            } else {
                // Since we don't have timers or interrupts yet, this is the end
                panic!("No task to schedule");
            }
        }
    }

    pub unsafe fn yield_cpu<System: Kernel>(&self)
    where
        System: Port<PortTaskState = TaskState>,
    {
        log::trace!("yield_cpu");
        assert!(!self.is_cpu_lock_active());

        let task = System::state().running_task().expect("no running task");
        task.port_task_state.yield_current(self);
    }

    pub unsafe fn enter_cpu_lock(&self) {
        log::trace!("enter_cpu_lock");
        assert!(!self.is_cpu_lock_active());
        self.cpu_lock.store(true, Ordering::Relaxed);
    }

    pub unsafe fn leave_cpu_lock(&self) {
        log::trace!("leave_cpu_lock");
        assert!(self.is_cpu_lock_active());
        self.cpu_lock.store(false, Ordering::Relaxed);
    }

    pub unsafe fn initialize_task_state<System: Kernel>(
        &self,
        task: &'static TaskCb<System, TaskState>,
    ) where
        System: Port<PortTaskState = TaskState>,
    {
        log::trace!("initialize_task_state {:p}", task);

        let pts = &task.port_task_state;
        match pts.tsm.load(Ordering::Relaxed) {
            TSM_DORMANT => {}
            TSM_RUNNING | TSM_RUNNABLE => {
                todo!("terminating a thread is not implemented yet");
            }
            TSM_UNINIT => {
                pts.tsm.store(TSM_DORMANT, Ordering::Relaxed);
            }
            _ => unreachable!(),
        }
    }

    pub fn is_cpu_lock_active(&self) -> bool {
        let b = self.cpu_lock.load(Ordering::Relaxed);
        log::trace!("is_cpu_lock_active -> {:?}", b);
        b
    }
}

#[macro_export]
macro_rules! use_port {
    (unsafe $vis:vis struct $sys:ident) => {
        $vis struct $sys;

        static PORT_STATE: $crate::State = $crate::State::new();

        // Assume `$sys: Kernel`
        unsafe impl $crate::Port for $sys {
            type PortTaskState = $crate::TaskState;
            const PORT_TASK_STATE_INIT: Self::PortTaskState = $crate::TaskState::new();

            unsafe fn dispatch_first_task() -> ! {
                PORT_STATE.dispatch_first_task::<Self>()
            }

            unsafe fn yield_cpu() {
                PORT_STATE.yield_cpu::<Self>()
            }

            unsafe fn enter_cpu_lock() {
                PORT_STATE.enter_cpu_lock()
            }

            unsafe fn leave_cpu_lock() {
                PORT_STATE.leave_cpu_lock()
            }

            unsafe fn initialize_task_state(task: &'static $crate::TaskCb<Self, Self::PortTaskState>) {
                PORT_STATE.initialize_task_state(task)
            }

            fn is_cpu_lock_active() -> bool {
                PORT_STATE.is_cpu_lock_active()
            }
        }

        fn main() {
            $crate::env_logger::init();

            // Safety: We are a port, so it's okay to call these
            unsafe {
                <$sys as $crate::PortToKernel>::boot();
            }
        }
    };
}
