initSidebarItems({"constant":[["TIME_HARD_HEADROOM","The extent of how overdue the firing of [`timer_tick`] can be without breaking the kernel timing algorithm."],["TIME_USER_HEADROOM","The extent of how overdue a timed event can be made or how far a timed event can be delayed past `Duration::MAX` by a call to [`adjust_time`]."]],"enum":[["ActivateTaskError","Error type for [`Task::activate`]."],["AdjustTimeError","Error type for [`Kernel::adjust_time`]."],["BoostPriorityError","Error type for [`Kernel::boost_priority`] and [`Kernel::unboost_priority`]."],["ClearInterruptLineError","Error type for [`InterruptLine::clear`]."],["CpuLockError","Error type for [`Kernel::acquire_cpu_lock`] and [`Kernel::release_cpu_lock`]."],["EnableInterruptLineError","Error type for [`InterruptLine::enable`] and [`InterruptLine::disable`]."],["ExitTaskError","Error type for [`Kernel::exit_task`]."],["GetCurrentTaskError","Error type for [`Task::current`]."],["GetEventGroupError","Error type for [`EventGroup::get`]."],["InterruptTaskError","Error type for [`Task::interrupt`]."],["ParkError","Error type for [`Kernel::park`]."],["ParkTimeoutError","Error type for [`Kernel::park_timeout`]."],["PendInterruptLineError","Error type for [`InterruptLine::pend`]."],["QueryInterruptLineError","Error type for [`InterruptLine::is_pending`]."],["QueueOrder","Specifies the sorting order of a wait queue."],["ResultCode","All result codes (including success) that the C API can return."],["SetInterruptLinePriorityError","Error type for [`InterruptLine::set_priority`] and [`InterruptLine::set_priority_unchecked`]."],["SetTaskPriorityError","Error type for [`Task::set_priority`]."],["SetTimerDelayError","Error type for [`Timer::set_delay`]."],["SetTimerPeriodError","Error type for [`Timer::set_period`]."],["SleepError","Error type for [`Kernel::sleep`]."],["StartTimerError","Error type for [`Timer::start`]."],["StopTimerError","Error type for [`Timer::stop`]."],["TimeError","Error type for [`Kernel::time`] and [`Kernel::set_time`]."],["UnparkError","Error type for [`Task::unpark`]."],["UnparkExactError","Error type for [`Task::unpark_exact`]."],["UpdateEventGroupError","Error type for [`EventGroup::set`] and [`EventGroup::clear`]."],["WaitError","Error type for wait operations such as [`EventGroup::wait`]."],["WaitEventGroupError","Error type for [`EventGroup::wait`]."],["WaitEventGroupTimeoutError","Error type for [`EventGroup::wait_timeout`]."],["WaitTimeoutError","Error type for wait operations with timeout such as [`EventGroup::wait_timeout`]."]],"mod":[["cfg","Static configuration mechanism for the kernel"]],"struct":[["EventGroup","Represents a single event group in a system."],["EventGroupWaitFlags","Options for [`EventGroup::wait`]."],["Hunk","Represents a single hunk in a system."],["InterruptHandler","Represents a registered (second-level) interrupt handler in a system."],["InterruptLine","Refers to an interrupt line in a system."],["StackHunk","[`Hunk`] for a task stack."],["StartupHook","Represents a registered startup hook in a system."],["State","Global kernel state."],["Task","Represents a single task in a system."],["TaskAttr","The static properties of a task."],["TaskCb","Task control block - the state data of a task."],["Timer","Represents a single timer in a system."]],"trait":[["Kernel","Provides access to the global API functions exposed by the kernel."],["KernelCfg1","Associates \"system\" types with kernel-private data. Use [`build!`] to implement."],["KernelCfg2","Associates \"system\" types with kernel-private data. Use [`build!`] to implement."],["Port","Represents a particular group of traits that a port should implement."],["PortInterrupts","Implemented by a port. This trait contains items related to controlling interrupt lines."],["PortThreading","Implemented by a port. This trait contains items related to low-level operations for controlling CPU states and context switching."],["PortTimer","Implemented by a port. This trait contains items related to controlling a system timer."],["PortToKernel","Methods intended to be called by a port."]],"type":[["AtomicEventGroupBits",""],["EventGroupBits","Unsigned integer type backing event groups."],["Id","Numeric value used to identify various kinds of kernel objects."],["InterruptNum","Numeric value used to identify interrupt lines."],["InterruptPriority","Priority value for an interrupt line."],["UTicks","Unsigned integer type representing a tick count used by [a port timer driver]. The period of each tick is fixed at one microsecond."]]});