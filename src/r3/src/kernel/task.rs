//! Tasks
#[cfg(feature = "priority_boost")]
use core::sync::atomic::Ordering;
use core::{convert::TryFrom, fmt, hash, marker::PhantomData, mem};
use num_traits::ToPrimitive;

use super::{
    hunk::Hunk, mutex, state, timeout, utils, wait, ActivateTaskError, BadIdError, ExitTaskError,
    GetCurrentTaskError, GetTaskPriorityError, Id, InterruptTaskError, Kernel, KernelCfg1,
    ParkError, ParkTimeoutError, PortThreading, SetTaskPriorityError, SleepError, UnparkError,
    UnparkExactError, WaitTimeoutError,
};
use crate::{time::Duration, utils::Init};

#[doc(hidden)]
pub mod readyqueue;
use self::readyqueue::Queue as _;

#[cfg_attr(doc, svgbobdoc::transform)]
/// Represents a single task in a system.
///
/// This type is ABI-compatible with [`Id`].
///
/// <div class="admonition-follows"></div>
///
/// > **Relation to Other Specifications:** Present in almost every real-time
/// > operating system.
///
/// # Task States
///
/// A task may be in one of the following states:
///
///  - **Dormant** — The task is not executing, doesn't have an associated
///    execution [thread], and can be [activated].
///
///  - **Ready** — The task has an associated execution thread, which is ready to
///    be scheduled to the CPU
///
///  - **Running** — The task has an associated execution thread, which is
///    currently scheduled to the CPU
///
///  - **Waiting** — The task has an associated execution thread, which is
///    currently blocked by a blocking operation
///
/// <center>
/// ```svgbob
///                     ,-------,
///    ,--------------->| Ready |<--------------,
///    |                '-------'               |
///    |          dispatch | ^                  |
///    |                   | |                  |
///    | release           | |                  | activate
/// ,---------,            | |           ,---------,
/// | Waiting |            | |           | Dormant |
/// '---------'            | |           '---------'
///    ^                   | |                  ^
///    |                   | |                  |
///    |                   v | preempt          |
///    |          wait ,---------,              |
///    '---------------| Running |--------------'
///                    '---------' exit
/// ```
/// </center>
///
/// [thread]: crate#threads
/// [activated]: Task::activate
#[doc(include = "../common.md")]
#[repr(transparent)]
pub struct Task<System>(Id, PhantomData<System>);

impl<System> Clone for Task<System> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<System> Copy for Task<System> {}

impl<System> PartialEq for Task<System> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<System> Eq for Task<System> {}

impl<System> hash::Hash for Task<System> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.0, state);
    }
}

impl<System> fmt::Debug for Task<System> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Task").field(&self.0).finish()
    }
}

impl<System> Task<System> {
    /// Construct a `Task` from `Id`.
    ///
    /// # Safety
    ///
    /// The kernel can handle invalid IDs without a problem. However, the
    /// constructed `Task` may point to an object that is not intended to be
    /// manipulated except by its creator. This is usually prevented by making
    /// `Task` an opaque handle, but this safeguard can be circumvented by
    /// this method.
    ///
    /// Constructing a `Task` for a current task is allowed. This can be safely
    /// done by [`Task::current`].
    pub const unsafe fn from_id(id: Id) -> Self {
        Self(id, PhantomData)
    }

    /// Get the raw `Id` value representing this task.
    pub const fn id(self) -> Id {
        self.0
    }
}

impl<System: Kernel> Task<System> {
    /// Get the current task (i.e., the task in the Running state).
    ///
    /// In a task context, this method returns the currently running task.
    ///
    /// In an interrupt context, the result is unreliable because scheduling is
    /// deferred until the control returns to a task, but the current interrupt
    /// handler could be interrupted by another interrrupt, which might do
    /// scheduling on return (whether this happens or not is unspecified).
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn current() -> Result<Option<Self>, GetCurrentTaskError> {
        let mut lock = utils::lock_cpu::<System>()?;
        let task_cb = if let Some(cb) = System::state().running_task(lock.borrow_mut()) {
            cb
        } else {
            return Ok(None);
        };

        // Calculate an `Id` from the task CB pointer
        let offset_bytes =
            task_cb as *const TaskCb<_> as usize - System::task_cb_pool().as_ptr() as usize;
        let offset = offset_bytes / mem::size_of::<TaskCb<System>>();

        // Safety: Constructing a `Task` for a current task is allowed
        let task = unsafe { Self::from_id(Id::new(offset as usize + 1).unwrap()) };

        Ok(Some(task))
    }

    fn task_cb(self) -> Result<&'static TaskCb<System>, BadIdError> {
        System::get_task_cb(self.0.get() - 1).ok_or(BadIdError::BadId)
    }

    /// Start the execution of the task.
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn activate(self) -> Result<(), ActivateTaskError> {
        let lock = utils::lock_cpu::<System>()?;
        let task_cb = self.task_cb()?;
        activate(lock, task_cb)
    }

    /// Interrupt any ongoing wait operations undertaken by the task.
    ///
    /// This method interrupt any ongoing system call that is blocking the task.
    /// The interrupted system call will return [`WaitError::Interrupted`] or
    /// [`WaitTimeoutError::Interrupted`].
    ///
    /// [`WaitError::Interrupted`]: crate::kernel::WaitError::Interrupted
    /// [`WaitTimeoutError::Interrupted`]: crate::kernel::WaitTimeoutError::Interrupted
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn interrupt(self) -> Result<(), InterruptTaskError> {
        let mut lock = utils::lock_cpu::<System>()?;
        let task_cb = self.task_cb()?;
        wait::interrupt_task(
            lock.borrow_mut(),
            task_cb,
            Err(WaitTimeoutError::Interrupted),
        )?;

        // The task is now awake, check dispatch
        unlock_cpu_and_check_preemption(lock);

        Ok(())
    }

    /// Make the task's token available, unblocking [`Kernel::park`] now or in
    /// the future.
    ///
    /// If the token is already available, this method will return without doing
    /// anything. Use [`Task::unpark_exact`] if you need to detect this
    /// condition.
    ///
    /// If the task is currently being blocked by `Kernel::park`, the token will
    /// be immediately consumed. Otherwise, it will be consumed on a next call
    /// to `Kernel::park`.
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn unpark(self) -> Result<(), UnparkError> {
        match self.unpark_exact() {
            Ok(()) | Err(UnparkExactError::QueueOverflow) => Ok(()),
            Err(UnparkExactError::BadContext) => Err(UnparkError::BadContext),
            Err(UnparkExactError::BadId) => Err(UnparkError::BadId),
            Err(UnparkExactError::BadObjectState) => Err(UnparkError::BadObjectState),
        }
    }

    /// Make *exactly* one new token available for the task, unblocking
    /// [`Kernel::park`] now or in the future.
    ///
    /// If the token is already available, this method will return
    /// [`UnparkExactError::QueueOverflow`]. Thus, this method will succeed
    /// only if it made *exactly* one token available.
    ///
    /// If the task is currently being blocked by `Kernel::park`, the token will
    /// be immediately consumed. Otherwise, it will be consumed on a next call
    /// to `Kernel::park`.
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn unpark_exact(self) -> Result<(), UnparkExactError> {
        let lock = utils::lock_cpu::<System>()?;
        let task_cb = self.task_cb()?;
        unpark_exact(lock, task_cb)
    }

    /// Set the task's base priority.
    ///
    /// A task's base priority is used to calculate its [effective priority].
    /// Tasks with lower effective priorities execute first. The base priority
    /// is reset to the initial value specified by [`CfgTaskBuilder::priority`]
    /// upon activation.
    ///
    /// [effective priority]: Self::effective_priority
    /// [`CfgTaskBuilder::priority`]: crate::kernel::cfg::CfgTaskBuilder::priority
    ///
    /// The value must be in range `0..`[`num_task_priority_levels`]. Otherwise,
    /// this method will return [`SetTaskPriorityError::BadParam`].
    ///
    /// The task shouldn't be in the Dormant state. Otherwise, this method will
    /// return [`SetTaskPriorityError::BadObjectState`].
    ///
    /// [`num_task_priority_levels`]: crate::kernel::cfg::CfgBuilder::num_task_priority_levels
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn set_priority(self, priority: usize) -> Result<(), SetTaskPriorityError> {
        let lock = utils::lock_cpu::<System>()?;
        let task_cb = self.task_cb()?;
        set_task_base_priority(lock, task_cb, priority)
    }

    /// Get the task's base priority.
    ///
    /// The task shouldn't be in the Dormant state. Otherwise, this method will
    /// return [`GetTaskPriorityError::BadObjectState`].
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn priority(self) -> Result<usize, GetTaskPriorityError> {
        let lock = utils::lock_cpu::<System>()?;
        let task_cb = self.task_cb()?;

        if *task_cb.st.read(&*lock) == TaskSt::Dormant {
            Err(GetTaskPriorityError::BadObjectState)
        } else {
            Ok(task_cb.base_priority.read(&*lock).to_usize().unwrap())
        }
    }

    /// Get the task's effective priority.
    ///
    /// The effective priority is calculated based on the task's [base priority]
    /// and can be temporarily raised by a [mutex locking protocol].
    ///
    /// [base priority]: Self::priority
    /// [mutex locking protocol]: crate::kernel::MutexProtocol
    ///
    /// The task shouldn't be in the Dormant state. Otherwise, this method will
    /// return [`GetTaskPriorityError::BadObjectState`].
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn effective_priority(self) -> Result<usize, GetTaskPriorityError> {
        let lock = utils::lock_cpu::<System>()?;
        let task_cb = self.task_cb()?;

        if *task_cb.st.read(&*lock) == TaskSt::Dormant {
            Err(GetTaskPriorityError::BadObjectState)
        } else {
            Ok(task_cb.effective_priority.read(&*lock).to_usize().unwrap())
        }
    }
}

/// [`Hunk`] for a task stack.
pub struct StackHunk<System>(Hunk<System>, usize);

// Safety: Safe code can't access the contents. Also, the port is responsible
// for making sure `StackHunk` is used in the correct way.
unsafe impl<System> Sync for StackHunk<System> {}

impl<System: Kernel> fmt::Debug for StackHunk<System> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("StackHunk").field(&self.0.as_ptr()).finish()
    }
}

// TODO: Preferably `StackHunk` shouldn't be `Clone` as it strengthens the
//       safety obligation of `StackHunk::from_hunk`.
impl<System> Clone for StackHunk<System> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<System> Copy for StackHunk<System> {}

// TODO: Should we allow zero-sized `StackHunk`?
impl<System> Init for StackHunk<System> {
    const INIT: Self = Self(Init::INIT, 0);
}

impl<System> StackHunk<System> {
    /// Construct a `StackHunk` from `Hunk`.
    ///
    /// # Safety
    ///
    /// The caller is responsible for making sure the region represented by
    /// `hunk` is solely used for a single task's stack.
    ///
    /// Also, `hunk` must be properly aligned for a stack region.
    pub const unsafe fn from_hunk(hunk: Hunk<System>, len: usize) -> Self {
        Self(hunk, len)
    }

    /// Get the inner `Hunk` and the length, consuming `self`.
    pub fn into_inner(self) -> (Hunk<System>, usize) {
        (self.0, self.1)
    }
}

impl<System: Kernel> StackHunk<System> {
    /// Get a raw pointer to the hunk's contents.
    pub fn as_ptr(&self) -> *mut [u8] {
        core::ptr::slice_from_raw_parts_mut(self.0.as_ptr(), self.1)
    }
}

/// *Task control block* - the state data of a task.
#[repr(C)]
pub struct TaskCb<
    System: PortThreading,
    PortTaskState: 'static = <System as PortThreading>::PortTaskState,
    TaskPriority: 'static = <System as KernelCfg1>::TaskPriority,
    TaskReadyQueueData: 'static = <<System as KernelCfg1>::TaskReadyQueue as readyqueue::Queue<
        System,
    >>::PerTaskData,
> {
    /// Get a reference to `PortTaskState` in the task control block.
    ///
    /// This is guaranteed to be placed at the beginning of the struct so that
    /// assembler code can refer to this easily.
    pub port_task_state: PortTaskState,

    /// The static properties of the task.
    pub attr: &'static TaskAttr<System, TaskPriority>,

    /// The task's base priority.
    pub(super) base_priority: utils::CpuLockCell<System, TaskPriority>,

    /// The task's effective priority. It's calculated based on `base_priority`
    /// and may be temporarily elevated by a mutex locking protocol.
    ///
    /// Given a set of mutexes held by the task `mutexes`, the value is
    /// calculated by the following pseudocode:
    ///
    /// ```rust,ignore
    /// task_cb.base_priority.min(mutexes.map(|mutex_cb| {
    ///     if let Some(ceiling) = mutex_cb.ceiling {
    ///         assert!(ceiling <= task_cb.base_priority);
    ///         ceiling
    ///     } else {
    ///         TaskPriority::MAX
    ///     }
    /// }).min())
    /// ```
    ///
    /// Many operations change the inputs of this calculation. We take care to
    /// ensure the recalculation of this value completes in constant-time (in
    /// regard to the number of held mutexes) for as many cases as possible.
    ///
    /// The effective priority determines the task's position within the task
    /// ready queue. You must call `TaskReadyQueue::reorder_task` after updating
    /// `effective_priority` of a task which is in Ready state.
    pub(super) effective_priority: utils::CpuLockCell<System, TaskPriority>,

    pub(super) st: utils::CpuLockCell<System, TaskSt>,

    /// A flag indicating whether the task has a park token or not.
    pub(super) park_token: utils::CpuLockCell<System, bool>,

    /// Allows `TaskCb` to participate in one of linked lists.
    ///
    ///  - In a `Ready` state, this forms the linked list headed by
    ///    [`State::task_ready_queue`].
    ///
    /// [`State::task_ready_queue`]: crate::kernel::State::task_ready_queue
    pub(super) ready_queue_data: TaskReadyQueueData,

    /// The wait state of the task.
    pub(super) wait: wait::TaskWait<System>,

    /// The last mutex locked by the task.
    pub(super) last_mutex_held: utils::CpuLockCell<System, Option<&'static mutex::MutexCb<System>>>,
}

impl<
        System: Kernel,
        PortTaskState: fmt::Debug + 'static,
        TaskPriority: fmt::Debug + 'static,
        TaskReadyQueueData: fmt::Debug + 'static,
    > fmt::Debug for TaskCb<System, PortTaskState, TaskPriority, TaskReadyQueueData>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TaskCb")
            .field("self", &(self as *const _))
            .field("port_task_state", &self.port_task_state)
            .field("attr", self.attr)
            .field("base_priority", &self.base_priority)
            .field("effective_priority", &self.effective_priority)
            .field("st", &self.st)
            .field("ready_queue_data", &self.ready_queue_data)
            .field("wait", &self.wait)
            .field(
                "last_mutex_held",
                // Don't print the content of the mutex. It'll be printed
                // somewhere else in the debug printing of `KernelDebugPrinter`.
                &self
                    .last_mutex_held
                    .debug_fmt_with(|x, f| x.map(|x| x as *const _).fmt(f)),
            )
            .field("park_token", &self.park_token)
            .finish()
    }
}

/// The static properties of a task.
pub struct TaskAttr<System, TaskPriority: 'static = <System as KernelCfg1>::TaskPriority> {
    /// The entry point of the task.
    ///
    /// # Safety
    ///
    /// This is only meant to be used by a kernel port, as a task entry point,
    /// not by user code. Using this in other ways may cause an undefined
    /// behavior.
    pub entry_point: unsafe fn(usize),

    /// The parameter supplied for `entry_point`.
    pub entry_param: usize,

    // FIXME: Ideally, `stack` should directly point to the stack region. But
    //        this is blocked by <https://github.com/rust-lang/const-eval/issues/11>
    /// The hunk representing the stack region for the task.
    pub stack: StackHunk<System>,

    /// The initial base priority of the task.
    pub priority: TaskPriority,
}

impl<System: Kernel, TaskPriority: fmt::Debug> fmt::Debug for TaskAttr<System, TaskPriority> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TaskAttr")
            .field("entry_point", &self.entry_point)
            .field("entry_param", &self.entry_param)
            .field("stack", &self.stack)
            .field("priority", &self.priority)
            .finish()
    }
}

/// Task state machine
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskSt {
    /// The task is in the Dormant state.
    Dormant,

    Ready,

    /// The task is in the Running state.
    Running,

    /// The task is in the Waiting state.
    Waiting,

    /// The task should be activated at startup. This will transition into
    /// `Ready` or `Running` before the first task is scheduled.
    PendingActivation,
}

impl Init for TaskSt {
    const INIT: Self = Self::Dormant;
}

/// Implements [`Kernel::exit_task`].
pub(super) unsafe fn exit_current_task<System: Kernel>() -> Result<!, ExitTaskError> {
    if !System::is_task_context() {
        return Err(ExitTaskError::BadContext);
    }

    // If CPU Lock is inactive, activate it.
    // TODO: If `is_cpu_lock_active() == true`, assert that it was an
    //       application that has the lock. It's illegal for it to be a
    //       kernel-owned CPU Lock.
    let mut lock = unsafe {
        if !System::is_cpu_lock_active() {
            System::enter_cpu_lock();
        }
        utils::assume_cpu_lock::<System>()
    };

    #[cfg(feature = "priority_boost")]
    {
        // If Priority Boost is active, deactivate it.
        System::state()
            .priority_boost
            .store(false, Ordering::Release);
    }

    let running_task = System::state().running_task(lock.borrow_mut()).unwrap();

    // Abandon mutexes, waking up the next waiters of the mutexes (if any)
    mutex::abandon_held_mutexes(lock.borrow_mut(), running_task);
    debug_assert!(running_task.last_mutex_held.read(&*lock).is_none());

    // Transition the current task to Dormant
    assert_eq!(*running_task.st.read(&*lock), TaskSt::Running);
    running_task.st.replace(&mut *lock, TaskSt::Dormant);

    // Erase `running_task`
    System::state().running_task.replace(&mut *lock, None);

    core::mem::forget(lock);

    // Safety: (1) The user of `exit_task` acknowledges that all preexisting
    // data on the task stack will be invalidated and has promised that this
    // will not cause any UBs. (2) CPU Lock active
    unsafe {
        System::exit_and_dispatch(running_task);
    }
}

/// Initialize a task at boot time.
pub(super) fn init_task<System: Kernel>(
    lock: utils::CpuLockGuardBorrowMut<'_, System>,
    task_cb: &'static TaskCb<System>,
) {
    if let TaskSt::PendingActivation = task_cb.st.read(&*lock) {
        // `PendingActivation` is equivalent to `Dormant` but serves as a marker
        // indicating tasks that should be activated by `init_task`.

        // Safety: CPU Lock active, the task is (essentially) in the Dormant state
        unsafe { System::initialize_task_state(task_cb) };

        // Safety: The previous state is PendingActivation (which is equivalent
        // to Dormant) and we just initialized the task state, so this is safe
        unsafe { make_ready(lock, task_cb) };
    }
}

/// Implements `Task::activate`.
fn activate<System: Kernel>(
    mut lock: utils::CpuLockGuard<System>,
    task_cb: &'static TaskCb<System>,
) -> Result<(), ActivateTaskError> {
    if *task_cb.st.read(&*lock) != TaskSt::Dormant {
        return Err(ActivateTaskError::QueueOverflow);
    }

    // Discard a park token if the task has one
    task_cb.park_token.replace(&mut *lock, false);

    // Safety: CPU Lock active, the task is in the Dormant state
    unsafe { System::initialize_task_state(task_cb) };

    // Reset the task priority
    task_cb
        .base_priority
        .replace(&mut *lock, task_cb.attr.priority);
    task_cb
        .effective_priority
        .replace(&mut *lock, task_cb.attr.priority);

    // Safety: The previous state is Dormant, and we just initialized the task
    // state, so this is safe
    unsafe { make_ready(lock.borrow_mut(), task_cb) };

    // If `task_cb` has a higher priority, perform a context switch.
    unlock_cpu_and_check_preemption(lock);

    Ok(())
}

/// Transition the task into the Ready state. This function doesn't do any
/// proper cleanup for a previous state. If the previous state is `Dormant`, the
/// caller must initialize the task state first by calling
/// `initialize_task_state`.
pub(super) unsafe fn make_ready<System: Kernel>(
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
    task_cb: &'static TaskCb<System>,
) {
    // Make the task Ready
    task_cb.st.replace(&mut *lock, TaskSt::Ready);

    // Insert the task to the ready queue.
    //
    // Safety: `task_cb` is not in the ready queue
    unsafe {
        <System>::state()
            .task_ready_queue
            .push_back_task(lock.into(), task_cb);
    }
}

/// Relinquish CPU Lock. After that, if there's a higher-priority task than
/// `running_task`, call `Port::yield_cpu`.
///
/// System services that transition a task into the Ready state should call
/// this before returning to the caller.
pub(super) fn unlock_cpu_and_check_preemption<System: Kernel>(
    mut lock: utils::CpuLockGuard<System>,
) {
    // If Priority Boost is active, treat the currently running task as the
    // highest-priority task.
    if System::is_priority_boost_active() {
        debug_assert_eq!(
            *System::state()
                .running_task(lock.borrow_mut())
                .unwrap()
                .st
                .read(&*lock),
            TaskSt::Running
        );
        return;
    }

    let prev_task_priority =
        if let Some(running_task) = System::state().running_task(lock.borrow_mut()) {
            if *running_task.st.read(&*lock) == TaskSt::Running {
                running_task
                    .effective_priority
                    .read(&*lock)
                    .to_usize()
                    .unwrap()
            } else {
                usize::MAX
            }
        } else {
            usize::MAX
        };

    let has_preempting_task = System::state()
        .task_ready_queue
        .has_ready_task_in_priority_range(lock.borrow_mut().into(), ..prev_task_priority);

    // Relinquish CPU Lock
    drop(lock);

    if has_preempting_task {
        // Safety: CPU Lock inactive
        unsafe { System::yield_cpu() };
    }
}

/// Implements `PortToKernel::choose_running_task`.
#[inline]
pub(super) fn choose_next_running_task<System: Kernel>(
    mut lock: utils::CpuLockGuardBorrowMut<System>,
) {
    // If Priority Boost is active, treat the currently running task as the
    // highest-priority task.
    if System::is_priority_boost_active() {
        // Blocking system calls aren't allowed when Priority Boost is active
        debug_assert_eq!(
            *System::state()
                .running_task(lock.borrow_mut())
                .unwrap()
                .st
                .read(&*lock),
            TaskSt::Running
        );
        return;
    }

    // The priority of `running_task`
    let prev_running_task = System::state().running_task(lock.borrow_mut());
    let prev_task_priority = if let Some(running_task) = prev_running_task {
        if *running_task.st.read(&*lock) == TaskSt::Running {
            running_task
                .effective_priority
                .read(&*lock)
                .to_usize()
                .unwrap()
        } else {
            usize::MAX // (2) see the discussion below
        }
    } else {
        usize::MAX // (1) see the discussion below
    };

    // Decide the next task to run
    //
    // The special value `prev_task_priority == usize::MAX` indicates that
    // (1) there is no running task, or (2) there was one but it is not running
    // anymore, and we need to elect a new task to run. In case (2), we would
    // want to update `running_task` regardless of whether there exists a
    // schedulable task or not. That is, even if there was not such a task, we
    // would still want to assign `None` to `running_task`. Therefore,
    // `pop_front_task` is designed to return `SwitchTo(None)` in this case.
    let decision = System::state()
        .task_ready_queue
        .pop_front_task(lock.borrow_mut().into(), prev_task_priority);

    let next_running_task = match decision {
        readyqueue::ScheduleDecision::SwitchTo(task) => task,

        // Return if there's no task willing to take over the current one, and
        // the current one can still run.
        readyqueue::ScheduleDecision::Keep => {
            // If `prev_task_priority == usize::MAX`, `pop_front_task` must
            // return `SwitchTo(_)`.
            debug_assert_ne!(prev_task_priority, usize::MAX);
            return;
        }
    };

    if let Some(task) = next_running_task {
        // Transition `next_running_task` into the Running state
        task.st.replace(&mut *lock, TaskSt::Running);

        if ptr_from_option_ref(prev_running_task) == task {
            // Skip the remaining steps if `task == prev_running_task`
            return;
        }
    }

    // `prev_running_task` now loses the control of the processor.
    if let Some(running_task) = prev_running_task {
        debug_assert_ne!(
            ptr_from_option_ref(prev_running_task),
            ptr_from_option_ref(next_running_task),
        );
        match running_task.st.read(&*lock) {
            TaskSt::Running => {
                // Transition `prev_running_task` into Ready state.
                // Safety: The previous state is Running, so this is safe
                unsafe { make_ready(lock.borrow_mut(), running_task) };
            }
            TaskSt::Waiting => {
                // `prev_running_task` stays in Waiting state.
            }
            TaskSt::Ready => {
                // `prev_running_task` stays in Ready state.
            }
            _ => unreachable!(),
        }
    }

    System::state()
        .running_task
        .replace(&mut *lock, next_running_task);
}

#[inline]
fn ptr_from_option_ref<T>(x: Option<&T>) -> *const T {
    if let Some(x) = x {
        x
    } else {
        core::ptr::null()
    }
}

/// Transition the currently running task into the Waiting state. Returns when
/// woken up.
///
/// The current context must be [waitable] (This function doesn't check
/// that). The caller should use `expect_waitable_context` to do that.
///
/// [waitable]: crate#contets
pub(super) fn wait_until_woken_up<System: Kernel>(
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
) {
    debug_assert_eq!(state::expect_waitable_context::<System>(), Ok(()));

    // Transition the current task to Waiting
    let running_task = System::state().running_task(lock.borrow_mut()).unwrap();
    assert_eq!(*running_task.st.read(&*lock), TaskSt::Running);
    running_task.st.replace(&mut *lock, TaskSt::Waiting);

    loop {
        // Temporarily release the CPU Lock before calling `yield_cpu`
        // Safety: (1) We don't access rseources protected by CPU Lock.
        //         (2) We currently have CPU Lock.
        //         (3) We will re-acquire a CPU Lock before returning from this
        //             function.
        unsafe { System::leave_cpu_lock() };

        // Safety: CPU Lock inactive
        unsafe { System::yield_cpu() };

        // Re-acquire a CPU Lock
        unsafe { System::enter_cpu_lock() };

        if *running_task.st.read(&*lock) == TaskSt::Running {
            break;
        }

        assert_eq!(*running_task.st.read(&*lock), TaskSt::Waiting);
    }
}

/// Implements [`Kernel::park`].
pub(super) fn park_current_task<System: Kernel>() -> Result<(), ParkError> {
    let mut lock = utils::lock_cpu::<System>()?;
    state::expect_waitable_context::<System>()?;

    let running_task = System::state().running_task(lock.borrow_mut()).unwrap();

    // If the task already has a park token, return immediately
    if running_task.park_token.replace(&mut *lock, false) {
        return Ok(());
    }

    // Wait until woken up by `unpark_exact`
    wait::wait_no_queue(lock.borrow_mut(), wait::WaitPayload::Park)?;

    Ok(())
}

/// Implements [`Kernel::park_timeout`].
pub(super) fn park_current_task_timeout<System: Kernel>(
    timeout: Duration,
) -> Result<(), ParkTimeoutError> {
    let time32 = timeout::time32_from_duration(timeout)?;
    let mut lock = utils::lock_cpu::<System>()?;
    state::expect_waitable_context::<System>()?;

    let running_task = System::state().running_task(lock.borrow_mut()).unwrap();

    // If the task already has a park token, return immediately
    if running_task.park_token.replace(&mut *lock, false) {
        return Ok(());
    }

    // Wait until woken up by `unpark_exact`
    wait::wait_no_queue_timeout(lock.borrow_mut(), wait::WaitPayload::Park, time32)?;

    Ok(())
}

/// Implements [`Task::unpark_exact`].
fn unpark_exact<System: Kernel>(
    mut lock: utils::CpuLockGuard<System>,
    task_cb: &'static TaskCb<System>,
) -> Result<(), UnparkExactError> {
    // Is the task currently parked?
    let is_parked = match task_cb.st.read(&*lock) {
        TaskSt::Dormant => return Err(UnparkExactError::BadObjectState),
        TaskSt::Waiting => wait::with_current_wait_payload(lock.borrow_mut(), task_cb, |payload| {
            matches!(payload, Some(wait::WaitPayload::Park))
        }),
        _ => false,
    };

    if is_parked {
        // Unblock the task. We confirmed that the task is in the Waiting state,
        // so `interrupt_task` should succeed.
        wait::interrupt_task(lock.borrow_mut(), task_cb, Ok(())).unwrap();

        // The task is now awake, check dispatch
        unlock_cpu_and_check_preemption(lock);

        Ok(())
    } else {
        // Put a park token
        if task_cb.park_token.replace(&mut *lock, true) {
            // It already had a park token
            Err(UnparkExactError::QueueOverflow)
        } else {
            Ok(())
        }
    }
}

/// Implements [`Kernel::sleep`].
pub(super) fn put_current_task_on_sleep_timeout<System: Kernel>(
    timeout: Duration,
) -> Result<(), SleepError> {
    let time32 = timeout::time32_from_duration(timeout)?;
    let mut lock = utils::lock_cpu::<System>()?;
    state::expect_waitable_context::<System>()?;

    // Wait until woken up by timeout
    match wait::wait_no_queue_timeout(lock.borrow_mut(), wait::WaitPayload::Sleep, time32) {
        Ok(_) => unreachable!(),
        Err(WaitTimeoutError::Interrupted) => Err(SleepError::Interrupted),
        Err(WaitTimeoutError::Timeout) => Ok(()),
    }
}

/// Implements [`Task::set_priority`].
fn set_task_base_priority<System: Kernel>(
    mut lock: utils::CpuLockGuard<System>,
    task_cb: &'static TaskCb<System>,
    base_priority: usize,
) -> Result<(), SetTaskPriorityError> {
    // Validate the given priority
    if base_priority >= System::NUM_TASK_PRIORITY_LEVELS {
        return Err(SetTaskPriorityError::BadParam);
    }
    let base_priority_internal =
        System::TaskPriority::try_from(base_priority).unwrap_or_else(|_| unreachable!());

    let st = *task_cb.st.read(&*lock);

    if st == TaskSt::Dormant {
        return Err(SetTaskPriorityError::BadObjectState);
    }

    let old_base_priority = task_cb.base_priority.read(&*lock).to_usize().unwrap();

    if old_base_priority == base_priority {
        return Ok(());
    }

    // Fail with `BadParam` if the operation would violate the precondition of
    // the locking protocol used in any of the held or waited mutexes. This
    // check is only needed when raising the priority.
    if base_priority < old_base_priority {
        // Get the currently-waited mutex (if any).
        let waited_mutex = wait::with_current_wait_payload(lock.borrow_mut(), task_cb, |payload| {
            if let Some(&wait::WaitPayload::Mutex(mutex_cb)) = payload {
                Some(mutex_cb)
            } else {
                None
            }
        });

        if let Some(waited_mutex) = waited_mutex {
            if !mutex::does_held_mutex_allow_new_task_base_priority(
                lock.borrow_mut(),
                waited_mutex,
                base_priority_internal,
            ) {
                return Err(SetTaskPriorityError::BadParam);
            }
        }

        // Check the precondition for all currently-held mutexes
        if !mutex::do_held_mutexes_allow_new_task_base_priority(
            lock.borrow_mut(),
            task_cb,
            base_priority_internal,
        ) {
            return Err(SetTaskPriorityError::BadParam);
        }
    }

    // Recalculate `effective_priority` according to the locking protocol
    // of held mutexes
    let effective_priority_internal =
        mutex::evaluate_task_effective_priority(lock.borrow_mut(), task_cb, base_priority_internal);
    let effective_priority = effective_priority_internal.to_usize().unwrap();

    // Assign the new priority
    task_cb
        .base_priority
        .replace(&mut *lock, base_priority_internal);
    let old_effective_priority = task_cb
        .effective_priority
        .replace(&mut *lock, effective_priority_internal)
        .to_usize()
        .unwrap();

    if old_effective_priority == effective_priority {
        return Ok(());
    }

    match st {
        TaskSt::Ready => unsafe {
            // Move the task within the ready queue
            //
            // Safety: `task_cb` was previously inserted to the ready queue
            // with an effective priority that is identical to
            // `old_effective_priority`.
            System::state().task_ready_queue.reorder_task(
                lock.borrow_mut().into(),
                task_cb,
                effective_priority,
                old_effective_priority,
            );
        },
        TaskSt::Running => {}
        TaskSt::Waiting => {
            // Reposition the task in a wait queue if the task is currently waiting
            wait::reorder_wait_of_task(lock.borrow_mut(), task_cb);
        }
        TaskSt::Dormant | TaskSt::PendingActivation => unreachable!(),
    }

    if let TaskSt::Running | TaskSt::Ready = st {
        // - If `st == TaskSt::Running`, `task_cb` is the currently running
        //   task. If the priority was lowered, it could be preempted by
        //   a task in the Ready state.
        // - If `st == TaskSt::Ready` and the priority was raised, it could
        //   preempt the currently running task.
        unlock_cpu_and_check_preemption(lock);
    }

    Ok(())
}
