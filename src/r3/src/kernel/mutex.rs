//! ~~Mutices~~ Mutexes
// FIXME: `debug_assert_matches` needing `assert_matches` is fixed by
//        <https://github.com/murarth/assert_matches/pull/9>
use assert_matches::{assert_matches, debug_assert_matches};
use core::{fmt, hash, marker::PhantomData};

use super::{
    state, task, timeout, utils,
    wait::{WaitPayload, WaitQueue},
    BadIdError, Id, Kernel, KernelCfg1, LockMutexError, LockMutexPrecheckError,
    LockMutexTimeoutError, MarkConsistentMutexError, PortThreading, QueryMutexError,
    TryLockMutexError, UnlockMutexError,
};
use crate::{time::Duration, utils::Init};

/// Specifies the locking protocol to be followed by a [mutex].
///
/// [mutex]: Mutex
///
/// <div class="admonition-follows"></div>
///
/// > **Relation to Other Specifications:** The operating systems and operating
/// > system specifications providing an interface for specifying a mutex
/// > protocol include (but are not limited to) the following: POSIX
/// > (`pthread_mutexattr_setprotocol` and `PTHREAD_PRIO_PROTECT`, etc.), RTEMS
/// > Classic API (`RTEMS_PRIORITY_CEILING`, etc.), and μITRON4.0 (`TA_CEILING`,
/// > etc.).
///
/// <div class="admonition-follows"></div>
///
/// > **Rationale:**
/// > When this enumerate type was added, the plan was to only support the
/// > priority ceiling protocol, so having a method
/// > `CfgMutexBuilder::ceiling_priority` taking a priority ceiling value would
/// > have been simpler. Nevertheless, it was decided to use this enumerate
/// > type to accomodate other protocols in the future and to allow specifying
/// > protocol-specific parameters.
#[doc(include = "../common.md")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MutexProtocol {
    /// Locking the mutex does not affect the priority of the owning task.
    None,
    /// Locking the mutex raises the effective priority of the owning task
    /// to the mutex's priority ceiling according to
    /// [the immediate priority ceiling protocol]. The inner value specifies the
    /// priority ceiling.
    ///
    /// The value must be in range `0..`[`num_task_priority_levels`].
    ///
    /// [`num_task_priority_levels`]: crate::kernel::cfg::CfgBuilder::num_task_priority_levels
    /// [the immediate priority ceiling protocol]: https://en.wikipedia.org/wiki/Priority_ceiling_protocol
    Ceiling(usize),
}

/// Represents a single mutex in a system.
///
/// This type is ABI-compatible with [`Id`].
///
/// Mutexes are similar to binary semaphores (semaphores restricted to one
/// permit at maximum) but differ in some ways, such as the inclusion of a
/// mechanism for preventing unbounded priority inversion.
///
/// When a mutex is locked, it is considered to be owned by the task while the
/// lock is held and can only be unlocked by the same task. This also means that
/// a mutex cannot be locked (even with a non-blocking operation) in a
/// [non-task context], where there is no task to hold the mutex.
///
/// [non-task context]: crate#contexts
///
/// See [`r3::sync::Mutex`] for a thread-safe container that uses this
/// `Mutex` internally to protect shared data from concurrent access.
///
/// [`r3::sync::Mutex`]: crate::sync::Mutex`
///
/// <div class="admonition-follows"></div>
///
/// > **Relation to Other Specifications:** Present in many general-purpose and
/// > real-time operating systems.
///
/// # Examples
///
/// ```rust
/// #![feature(const_fn)]
/// #![feature(const_mut_refs)]
/// use r3::kernel::{
///     Kernel, LockMutexError, Mutex, MutexProtocol, Task, cfg::CfgBuilder,
/// };
///
/// struct Objects<System> {
///     mutex: Mutex<System>,
/// }
///
/// const fn configure<System: Kernel>(b: &mut CfgBuilder<System>) -> Objects<System> {
///     let mutex = Mutex::build()
///         .protocol(MutexProtocol::Ceiling(1))
///         .finish(b);
///     Objects { mutex }
/// }
///
/// fn hoge<System: Kernel>(app: &Objects<System>) {
///     match app.mutex.lock() {
///         Ok(()) => {},
///         Err(LockMutexError::Abandoned) => {
///             app.mutex.mark_consistent().unwrap();
///         }
///         Err(e) => panic!("failed to lock the mutex: {:?}", e),
///     }
///     app.mutex.unlock().unwrap();
/// }
/// ```
///
/// # Robustness
///
/// If a task exits while holding a mutex, the mutex is considered to be
/// *abandoned*. An abandoned mutex can still be locked, but the lock function
/// will return `Err(Abandoned)`. **Note that the calling task will receive the
/// ownership of the mutex in this case.** The abandonment state will last until
/// [`Mutex::mark_consistent`] is called on the mutex.
///
/// When a task exits while holding more than one mutex, the order in which the
/// mutexes are abandoned is not specified.
///
/// <div class="admonition-follows"></div>
///
/// > <details>
/// > <summary>Relation to Other Specifications</summary>
/// >
/// > This behavior is based on robust mutexes from POSIX.1-2008
/// > (`PTHREAD_MUTEX_ROBUST`) with one difference:
/// > A mutex never falls into an irrecoverable state — [`Mutex::lock`] would
/// > repeatedly return `Err(Abandoned)` until [`Mutex::mark_consistent`] is
/// > called. This change reduces the internal state bits and the complexity of
/// > the internal logic not to punish normal usage too much. It also loosely
/// > imitates the poisoning semantics of `std::sync::Mutex`.
/// >
/// > A [Win32 mutex] incorporates a flag indicating if the mutex has been
/// > abandoned. An abandoned mutex can be locked as usual, but the wait
/// > function will return `WAIT_ABANDONED`. The flag is cleared automatically,
/// > i.e., unlike POSIX, an abandoned mutex doesn't have to be explicitly
/// > marked consistent.
/// >
/// > In μITRON4.0 and μT-Kernel, abandoned mutexes are implicitly unlocked.
/// >
/// > All of the other operating systems' behavior described above can be
/// > emulated by having a per-mutex flag and performing additional tasks in the
/// > API translation layer.
/// >
/// > </details>
///
/// [Win32 mutex]: https://docs.microsoft.com/en-us/windows/win32/sync/mutex-objects
///
/// <div class="admonition-follows"></div>
///
/// > <details>
/// > <summary>Rationale</summary>
/// >
/// > Every customization option brings an additional overhead.
/// > The overhead introduced by the robustness is likely to outweigh the
/// > overhead to provide choices. Therefore, we decided not to add an attribute
/// > to control the robustness.
/// >
/// > We desired a predictable behavior in as many cases a possible, which
/// > excludes the option of leaving the behavior undefined. Failing to unlock
/// > a mutex usually indicates a serious programming error. A future version of
/// > R3 might include functionality to terminate an arbitrary task,
/// > e.g., to respond to a fatal condition such as panicking and a bus error by
/// > containing the fault to the faulting task. In these cases, the data
/// > protected by an abandoned mutex may be left in an inconsistent state
/// > and should be restored to a consistent state before it can be safely
/// > accessed again. To ensure this recommendation is followed correctly
/// > (unless explicitly opted out), we decided to make the robustness the
/// > default behavior.
/// >
/// > </details>
///
/// # Locking Protocols
///
/// `Mutex` supports [the immediate priority ceiling protocol] to avoid
/// unbounded [priority inversion].
///
/// A locking protocol can be chosen by [`CfgMutexBuilder::protocol`].
/// Additional information can be found at [`MutexProtocol`].
///
/// [the immediate priority ceiling protocol]: https://en.wikipedia.org/wiki/Priority_ceiling_protocol
/// [priority inversion]: https://en.wikipedia.org/wiki/Priority_inversion
/// [`CfgMutexBuilder::protocol`]: crate::kernel::cfg::CfgMutexBuilder::protocol
///
/// <div class="admonition-follows"></div>
///
/// > <details>
/// > <summary>Relation to Other Specifications</summary>
/// >
/// > POSIX supports specifying a locking protocol by
/// > `pthread_mutexattr_setprotocol`. The following protocols are supported:
/// > `PTHREAD_PRIO_NONE` (none), `PTHREAD_PRIO_INHERIT`
/// > ([the priority inheritance protocol]), and `PTHREAD_PRIO_PROTECT` (the
/// > immediate priority ceiling protocol).
/// >
/// > μITRON4.0 supports both the priority inheritance protocol and the
/// > immediate priority ceiling protocol. It permits an implementation to
/// > adhere to the simplified priority control rule, which lowers a task's
/// > effective priority only when the task unlocks the last mutex lock held by
/// > the task.
/// >
/// > [Mutexes in ChibiOS/RT] implements the priority inheritance protocol.
/// > Unlock operations must always be performed in lock-reverse order. This
/// > restriction is required for an efficient implementation of the priority
/// > inheritance protocol.
/// >
/// > [Mutexes in ChibiOS/RT]: http://chibios.sourceforge.net/docs3/rt/group__mutexes.html
/// >
/// > Mutexes in the TOPPERS next generation and third generation kernels only
/// > support the immediate priority ceiling protocol. The third generation
/// > kernels further restrict the unlock order to be a lock-reverse order.
/// >
/// > The following table summaries the properties of mutexes in each operating
/// > system or operating system specification.
/// >
/// > |  Specification   | PI  | PC  | Unlock Order | Lower Priority |
/// > | ---------------- | --- | --- | ------------ | -------------- |
/// > | ChibiOS/RT       | yes | no  | lock-reverse | immediate      |
/// > | FreeRTOS         | yes | no  | arbitrary    | last mutex     |
/// > | POSIX            | yes | yes | arbitrary    | immediate      |
/// > | RTEMS            | yes | yes | arbitrary    | last mutex     |
/// > | TOPPERS 3rd Gen  | no  | yes | lock-reverse | immediate      |
/// > | TOPPERS Next Gen | no  | yes | arbitrary    | immediate      |
/// > | VxWorks          | yes | yes | arbitrary    | ?              |
/// > | μITRON4.0        | yes | yes | arbitrary    |                |
/// > | **R3**           | no  | yes | lock-reverse | immediate      |
/// >
/// >  - The **PI** column indicates the availability of
/// >    [the priority inheritance protocol].
/// >
/// >  - The **PC** column indicates the availability of the priority ceiling
/// >    protocol.
/// >
/// >  - The **Unlock Order** column indicates any restrictions imposed on
/// >    the unlocking order.
/// >
/// >  - The **Lower Priority** column indicates whether an owning task's
/// >    priority may be lowered whenever it unlocks a mutex or only when it
/// >    unlocks the last mutex held.
/// >
/// > </details>
///
/// [the priority inheritance protocol]: https://en.wikipedia.org/wiki/Priority_inheritance
///
/// <div class="admonition-follows"></div>
///
/// > <details>
/// > <summary>Rationale</summary>
/// >
/// > There are numerous reasons that led to the decision not to implement the
/// > priority inheritance protocol.
/// >
/// >  - We couldn't afford time to implement and test both protocols at this
/// >    time.
/// >    The entire project is at a prototyping stage, so we would better
/// >    implement the other one when there is an actual need for it.
/// >
/// >  - There are many arguments against using the priority inheritance
/// >    protocol in real-time systems, although they are somewhat out-dated.
/// >
/// >    Victor Yodaiken. “Against priority inheritance.” (2004):
/// >
/// >    > The RTLinux core does not support priority inheritance for a simple
/// >    > reason: priority inheritance is incompatible with reliable real-time
/// >    > system design. Priority inheritance is neither efficient nor
/// >    > reliable. Implementations are either incomplete (and unreliable) or
/// >    > surprisingly complex and intrusive. In fact, the original academic
/// >    > paper presenting priority inheritance \[3\] specifies (and “proves
/// >    > correct”) an inheritance algorithm that is wrong. Worse, the basic
/// >    > intent of the mechanism is to compensate for writing real-time
/// >    > software without taking care of the interaction between priority and
/// >    > mutual exclusion. All too often the result will be incorrect software
/// >    > with errors that are hard to find during test.
/// >
/// >    > Inheritance algorithms are complicated and easy to get wrong. In
/// >    > practice putting priority inheritance into an operating system
/// >    > increases the inversion delays produced by the operating system.
/// >
/// >    > The VxWorks designers originally tried to evade the issue by having a
/// >    > thread retain its highest inherited priority until it released all
/// >    > locks — but this can cause unbounded inversion.
/// >
/// >    Uresh Vahalia. *Unix Internals: The New Frontiers*. Prentice-Hall,
/// >    1996:
/// >
/// >    > Priority inheritance reduces the amount of time a high-priority
/// >    > process must block on resources held by lower-priority processes. The
/// >    > worst-case delay, however, is still much greater than what is
/// >    > acceptable for many real-time applications. One reason is that the
/// >    > blocking chain can grow arbitrarily long.
/// >
/// > We decided to restrict the unlocking order to a lock-reverse order to
/// > minimize the cost of maintaining the list of mutexes held by a task.
/// >
/// > </details>
///
#[doc(include = "../common.md")]
#[repr(transparent)]
pub struct Mutex<System>(Id, PhantomData<System>);

impl<System> Clone for Mutex<System> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<System> Copy for Mutex<System> {}

impl<System> PartialEq for Mutex<System> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<System> Eq for Mutex<System> {}

impl<System> hash::Hash for Mutex<System> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.0, state);
    }
}

impl<System> fmt::Debug for Mutex<System> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Mutex").field(&self.0).finish()
    }
}

impl<System> Mutex<System> {
    /// Construct a `Mutex` from `Id`.
    ///
    /// # Safety
    ///
    /// The kernel can handle invalid IDs without a problem. However, the
    /// constructed `Mutex` may point to an object that is not intended to be
    /// manipulated except by its creator. This is usually prevented by making
    /// `Mutex` an opaque handle, but this safeguard can be circumvented by
    /// this method.
    pub const unsafe fn from_id(id: Id) -> Self {
        Self(id, PhantomData)
    }

    /// Get the raw `Id` value representing this mutex.
    pub const fn id(self) -> Id {
        self.0
    }
}

impl<System: Kernel> Mutex<System> {
    fn mutex_cb(self) -> Result<&'static MutexCb<System>, BadIdError> {
        System::get_mutex_cb(self.0.get() - 1).ok_or(BadIdError::BadId)
    }

    /// Get a flag indicating whether the mutex is currently locked.
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn is_locked(self) -> Result<bool, QueryMutexError> {
        let lock = utils::lock_cpu::<System>()?;
        let mutex_cb = self.mutex_cb()?;
        Ok(mutex_cb.owning_task.get(&*lock).is_some())
    }

    /// Unlock the mutex.
    ///
    /// Mutexes must be unlocked in a lock-reverse order, or this method will
    /// return [`UnlockMutexError::BadObjectState`].
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn unlock(self) -> Result<(), UnlockMutexError> {
        let lock = utils::lock_cpu::<System>()?;
        state::expect_waitable_context::<System>()?;
        let mutex_cb = self.mutex_cb()?;

        unlock_mutex(mutex_cb, lock)
    }

    /// Acquire the mutex, blocking the current thread until it is able to do
    /// so.
    ///
    /// An [abandoned mutex] can still be locked, but this method will return
    /// `Err(Abandoned)`. **Note that the current task will receive the
    /// ownership of the mutex even in this case.**
    ///
    /// [abandoned mutex]: #robustness
    ///
    /// This system service may block. Therefore, calling this method is not
    /// allowed in [a non-waitable context] and will return `Err(BadContext)`.
    ///
    /// [a non-waitable context]: crate#contexts
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn lock(self) -> Result<(), LockMutexError> {
        let lock = utils::lock_cpu::<System>()?;
        state::expect_waitable_context::<System>()?;
        let mutex_cb = self.mutex_cb()?;

        lock_mutex(mutex_cb, lock)
    }

    /// [`lock`](Self::lock) with timeout.
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn lock_timeout(self, timeout: Duration) -> Result<(), LockMutexTimeoutError> {
        let time32 = timeout::time32_from_duration(timeout)?;
        let lock = utils::lock_cpu::<System>()?;
        state::expect_waitable_context::<System>()?;
        let mutex_cb = self.mutex_cb()?;

        lock_mutex_timeout(mutex_cb, lock, time32)
    }

    /// Non-blocking version of [`lock`](Self::lock). Returns
    /// immediately with [`TryLockMutexError::Timeout`] if the unblocking
    /// condition is not satisfied.
    ///
    /// Note that unlike [`Semaphore::poll_one`], this operation is disallowed
    /// in a non-task context because a mutex lock needs an owning task.
    ///
    /// [`Semaphore::poll_one`]: crate::kernel::Semaphore::poll_one
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn try_lock(self) -> Result<(), TryLockMutexError> {
        let lock = utils::lock_cpu::<System>()?;
        state::expect_task_context::<System>()?;
        let mutex_cb = self.mutex_cb()?;

        try_lock_mutex(mutex_cb, lock)
    }

    /// Mark the state protected by the mutex as consistent.
    ///
    /// <div class="admonition-follows"></div>
    ///
    /// > **Relation to Other Specifications:** Equivalent to
    /// > `pthread_mutex_consistent` from POSIX.1-2008.
    ///
    #[cfg_attr(not(feature = "inline_syscall"), inline(never))]
    pub fn mark_consistent(self) -> Result<(), MarkConsistentMutexError> {
        let mut lock = utils::lock_cpu::<System>()?;
        let mutex_cb = self.mutex_cb()?;

        if mutex_cb.inconsistent.replace(&mut *lock, false) {
            Ok(())
        } else {
            Err(MarkConsistentMutexError::BadObjectState)
        }
    }
}

/// *Mutex control block* - the state data of a mutex.
#[doc(hidden)]
pub struct MutexCb<
    System: PortThreading,
    TaskPriority: 'static = <System as KernelCfg1>::TaskPriority,
> {
    pub(super) ceiling: Option<TaskPriority>,

    pub(super) inconsistent: utils::CpuLockCell<System, bool>,

    pub(super) wait_queue: WaitQueue<System>,

    /// The next element in the singly-linked list headed by
    /// `TaskCb::last_mutex_held`, containing all mutexes currently held by the
    /// task.
    pub(super) prev_mutex_held: utils::CpuLockCell<System, Option<&'static Self>>,

    /// The task that currently owns the mutex lock.
    pub(super) owning_task: utils::CpuLockCell<System, Option<&'static task::TaskCb<System>>>,
}

impl<System: PortThreading> Init for MutexCb<System> {
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = Self {
        ceiling: Init::INIT,
        inconsistent: Init::INIT,
        wait_queue: Init::INIT,
        prev_mutex_held: Init::INIT,
        owning_task: Init::INIT,
    };
}

impl<System: Kernel> fmt::Debug for MutexCb<System> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MutexCb")
            .field("self", &(self as *const _))
            .field("ceiling", &self.ceiling)
            .field("inconsistent", &self.inconsistent)
            .field("wait_queue", &self.wait_queue)
            .field(
                "prev_mutex_held",
                // prevent O((# of held mutexes)²)-order debug printing
                &self
                    .prev_mutex_held
                    .debug_fmt_with(|x, f| x.map(|x| x as *const _).fmt(f)),
            )
            .field(
                "owning_task",
                // break infinite recursion (TaskCb → MutxeCb → TaskCb → ...)
                &self
                    .owning_task
                    .debug_fmt_with(|x, f| x.map(|x| x as *const _).fmt(f)),
            )
            .finish()
    }
}

/// Check the error conditions covered by [`LockMutexPrecheckError`].
///
///  - `WouldDeadlock`: The current task already owns the mutex.
///
///  - `BadParam`: The mutex was created with the protocol attribute having the
///    value `Ceiling` and the current task's priority is higher than the
///    mutex's priority ceiling.
///
/// Returns the currently running task for convenience of the caller.
#[inline]
fn precheck_and_get_running_task<System: Kernel>(
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
    mutex_cb: &'static MutexCb<System>,
) -> Result<&'static task::TaskCb<System>, LockMutexPrecheckError> {
    let task = System::state().running_task(lock.borrow_mut()).unwrap();

    if ptr_from_option_ref(mutex_cb.owning_task.get(&*lock)) == task {
        return Err(LockMutexPrecheckError::WouldDeadlock);
    }

    if let Some(ceiling) = mutex_cb.ceiling {
        if ceiling > task.base_priority.get(&*lock) {
            return Err(LockMutexPrecheckError::BadParam);
        }
    }

    Ok(task)
}

/// Check if the specified mutex, which is currently held or waited by a task,
/// is compatible with the new task base priority according to the mutex's
/// locking protocol.
///
/// The check is only needed when raising the priority.
#[inline]
pub(super) fn does_held_mutex_allow_new_task_base_priority<System: Kernel>(
    _lock: utils::CpuLockGuardBorrowMut<'_, System>,
    mutex_cb: &'static MutexCb<System>,
    new_base_priority: System::TaskPriority,
) -> bool {
    if let Some(ceiling) = mutex_cb.ceiling {
        if ceiling > new_base_priority {
            return false;
        }
    }

    true
}

/// Check if the task's held mutexes are all compatible with the new task base
/// priority according to the mutxes's locking protocols.
///
/// The check is only needed when raising the priority.
#[inline]
pub(super) fn do_held_mutexes_allow_new_task_base_priority<System: Kernel>(
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
    task: &'static task::TaskCb<System>,
    new_base_priority: System::TaskPriority,
) -> bool {
    let mut maybe_mutex_cb = task.last_mutex_held.get(&*lock);
    while let Some(mutex_cb) = maybe_mutex_cb {
        if !does_held_mutex_allow_new_task_base_priority(
            lock.borrow_mut(),
            mutex_cb,
            new_base_priority,
        ) {
            return false;
        }

        maybe_mutex_cb = mutex_cb.prev_mutex_held.get(&*lock);
    }
    true
}

/// Reevaluate the task's effective priority and return the result.
/// (This method doesn't update [`task::TaskCb::effective_priority`]).
/// The base priority is assumed to be `base_priority`.
pub(super) fn evaluate_task_effective_priority<System: Kernel>(
    lock: utils::CpuLockGuardBorrowMut<'_, System>,
    task: &'static task::TaskCb<System>,
    base_priority: System::TaskPriority,
) -> System::TaskPriority {
    let mut effective_priority = base_priority;
    let mut maybe_mutex_cb = task.last_mutex_held.get(&*lock);

    while let Some(mutex_cb) = maybe_mutex_cb {
        if let Some(ceiling) = mutex_cb.ceiling {
            effective_priority = effective_priority.min(ceiling);
        }

        maybe_mutex_cb = mutex_cb.prev_mutex_held.get(&*lock);
    }

    effective_priority
}

/// Check if the current state of a mutex satisfies the wait
/// condition.
///
/// If it satisfies the wait condition, this function updates it and
/// returns `true`. Otherwise, it returns `false`, indicating the calling task
/// should be blocked.
#[inline]
fn poll_core<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    running_task: &'static task::TaskCb<System>,
    lock: utils::CpuLockGuardBorrowMut<'_, System>,
) -> bool {
    if mutex_cb.owning_task.get(&*lock).is_some() {
        false
    } else {
        lock_core(mutex_cb, running_task, lock);
        true
    }
}

/// Give the ownership of the mutex to `task`.
///
/// The task must be in Running or Waiting state.
#[inline]
// FIXME: The extra parentheses in `debug_assert_matches!` are needed until
//        <https://github.com/murarth/assert_matches/pull/10> is fixed
#[allow(unused_parens)]
fn lock_core<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    task: &'static task::TaskCb<System>,
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
) {
    debug_assert_matches!(
        task.st.read(&*lock),
        (task::TaskSt::Running | task::TaskSt::Waiting)
    );

    mutex_cb.owning_task.replace(&mut *lock, Some(task));

    // Push `mutex_cb` to the list of the mutexes held by the task.
    let prev_mutex_held = task.last_mutex_held.replace(&mut *lock, Some(mutex_cb));
    mutex_cb
        .prev_mutex_held
        .replace(&mut *lock, prev_mutex_held);

    if let Some(ceiling) = mutex_cb.ceiling {
        let effective_priority = task.effective_priority.write(&mut *lock);
        *effective_priority = (*effective_priority).min(ceiling);
    }
}

fn lock_mutex<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    mut lock: utils::CpuLockGuard<System>,
) -> Result<(), LockMutexError> {
    let running_task = precheck_and_get_running_task(lock.borrow_mut(), mutex_cb)?;

    if !poll_core(mutex_cb, running_task, lock.borrow_mut()) {
        // The current state does not satify the wait condition. In this case,
        // start waiting. The wake-upper is responsible for using `poll_core`
        // to complete the effect of the wait operation.
        mutex_cb
            .wait_queue
            .wait(lock.borrow_mut(), WaitPayload::Mutex(mutex_cb))?;
    }

    if mutex_cb.inconsistent.get(&*lock) {
        Err(LockMutexError::Abandoned)
    } else {
        Ok(())
    }
}

fn try_lock_mutex<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    mut lock: utils::CpuLockGuard<System>,
) -> Result<(), TryLockMutexError> {
    let running_task = precheck_and_get_running_task(lock.borrow_mut(), mutex_cb)?;

    if !poll_core(mutex_cb, running_task, lock.borrow_mut()) {
        return Err(TryLockMutexError::Timeout);
    }

    if mutex_cb.inconsistent.get(&*lock) {
        Err(TryLockMutexError::Abandoned)
    } else {
        Ok(())
    }
}

fn lock_mutex_timeout<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    mut lock: utils::CpuLockGuard<System>,
    time32: timeout::Time32,
) -> Result<(), LockMutexTimeoutError> {
    let running_task = precheck_and_get_running_task(lock.borrow_mut(), mutex_cb)?;

    if !poll_core(mutex_cb, running_task, lock.borrow_mut()) {
        // The current state does not satify the wait condition. In this case,
        // start waiting. The wake-upper is responsible for using `poll_core`
        // to complete the effect of the wait operation.
        mutex_cb.wait_queue.wait_timeout(
            lock.borrow_mut(),
            WaitPayload::Mutex(mutex_cb),
            time32,
        )?;
    }

    if mutex_cb.inconsistent.get(&*lock) {
        Err(LockMutexTimeoutError::Abandoned)
    } else {
        Ok(())
    }
}

fn unlock_mutex<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    mut lock: utils::CpuLockGuard<System>,
) -> Result<(), UnlockMutexError> {
    let task = System::state().running_task(lock.borrow_mut()).unwrap();

    if ptr_from_option_ref(mutex_cb.owning_task.get(&*lock)) != task {
        // The current task does not currently own the mutex.
        return Err(UnlockMutexError::NotOwner);
    }

    if ptr_from_option_ref(task.last_mutex_held.get(&*lock)) != mutex_cb {
        // The correct mutex unlocking order is violated.
        return Err(UnlockMutexError::BadObjectState);
    }

    // Remove `mutex_cb` from the list of the mutexes held by the task.
    let prev_mutex_held = mutex_cb.prev_mutex_held.get(&*lock);
    task.last_mutex_held.replace(&mut *lock, prev_mutex_held);

    // Lower the task's effective priority. This may cause preemption.
    let base_priority = task.base_priority.get(&*lock);
    let effective_priority =
        evaluate_task_effective_priority(lock.borrow_mut(), task, base_priority);
    task.effective_priority
        .replace(&mut *lock, effective_priority);

    // Wake up the next waiter
    unlock_mutex_unchecked(mutex_cb, lock.borrow_mut());

    task::unlock_cpu_and_check_preemption(lock);

    Ok(())
}

/// Abandoon all mutexes held by the task.
///
/// This method doesn't restore the task's effective priority.
///
/// This method may make a task Ready, but doesn't yield the processor.
/// Call `unlock_cpu_and_check_preemption` (or something similar) as needed.
pub(super) fn abandon_held_mutexes<System: Kernel>(
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
    task: &'static task::TaskCb<System>,
) {
    let mut maybe_mutex_cb = task.last_mutex_held.replace(&mut *lock, None);
    while let Some(mutex_cb) = maybe_mutex_cb {
        maybe_mutex_cb = mutex_cb.prev_mutex_held.get(&*lock);
        mutex_cb.inconsistent.replace(&mut *lock, true);
        unlock_mutex_unchecked(mutex_cb, lock.borrow_mut());
    }
}

/// Wake up the next waiter of the mutex.
///
/// This method doesn't restore the task's effective priority.
///
/// This method may make a task Ready, but doesn't yield the processor.
/// Call `unlock_cpu_and_check_preemption` (or something similar) if it returns
/// `true`.
fn unlock_mutex_unchecked<System: Kernel>(
    mutex_cb: &'static MutexCb<System>,
    mut lock: utils::CpuLockGuardBorrowMut<'_, System>,
) {
    // Check if there's any other tasks waiting on the mutex
    if let Some(next_task) = mutex_cb.wait_queue.first_waiting_task(lock.borrow_mut()) {
        // Give the ownership of the mutex to `next_task`
        lock_core(mutex_cb, next_task, lock.borrow_mut());

        // Wake up the next waiter
        assert!(mutex_cb.wait_queue.wake_up_one(lock.borrow_mut()));
    } else {
        // There's no one waiting
        mutex_cb.owning_task.replace(&mut *lock, None);
    }
}

#[inline]
fn ptr_from_option_ref<T>(x: Option<&T>) -> *const T {
    if let Some(x) = x {
        x
    } else {
        core::ptr::null()
    }
}
