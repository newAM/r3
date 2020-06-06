//! Static configuration mechanism for the kernel
use core::{cell::UnsafeCell, marker::PhantomData, mem, num::NonZeroUsize};

use super::{hunk, task, Port};
use crate::utils::{AssertSendSync, Init};

mod vec;
#[doc(hidden)]
pub use self::vec::ComptimeVec;

/// Define a configuration function.
///
/// The following macros are available inside the function:
///
/// # `call!(expr, arg1, arg2, ...)`
///
/// Invokes another configuration function `expr`.
///
/// # `build!(expr, name1 = arg1, name2 = arg2, ...)`
///
/// Invokes a builder method `expr`, calls modifying methods `name1, name2, ...`
/// on the builder, and then finally calls `finish`, which is assumed to be a
/// configuration function.
///
/// # `new_task!(start = ENTRY_FN, ...)`
///
/// Defines a task. The following options are available:
///
///  - `start = ENTRY_FN: fn(usize)` specifies the task's entry point.
///  - `param = PARAM: usize` specifies the parameter to `start`.
///  - `statck_size = LEN: usize` specifies the task's stack size.
///  - `statck_hunk = HUNK: Hunk<System, [UnsafeCell<u8>]>` specifies the task's
///    hunk.
///
/// # `new_hunk!(T)`
///
/// Defines a new hunk. `T` must implement [`Init`](crate::utils::Init).
///
/// # `new_hunk!([u8], len = LEN, align = ALIGN)`
///
/// Defines a new zero-initialized hunk of the specified size and alignment.
///
#[macro_export]
macro_rules! configure {
    (
        [$dollar:tt]
        $( #[$meta:meta] )*
        $vis:vis fn $ident:ident(_: CfgBuilder<$sys:ty>) -> $id_map:ty {
            $($tt:tt)*
        }
    ) => {
        // FIXME: `&mut` in `const fn` <https://github.com/rust-lang/rust/issues/57349>
        //        is not implemented yet. Receiving `CfgBuilder` by `&mut _`
        //        would be more cleaner
        $( #[$meta] )*
        #[allow(unused_macros)]
        $vis const fn $ident(
            cfg: $crate::kernel::CfgBuilder<$sys>
        ) -> $crate::kernel::CfgOutput<$sys, $id_map> {
            #[allow(unused_mut)]
            let mut cfg = cfg;

            macro_rules! call {
                ($path:expr $dollar(, $arg:expr)* $dollar(,)*) => {{
                    use $crate::kernel::CfgOutput;

                    let CfgOutput { cfg: new_cfg, id_map } = $path(cfg, $dollar($arg),*);
                    cfg = new_cfg;
                    id_map
                }};
            }

            macro_rules! build {
                ($path:expr $dollar(, $argname:ident = $arg:expr)* $dollar(,)*) => {{
                    use $crate::kernel::CfgOutput;

                    let builder = $path $dollar(. $argname($arg))*;
                    let CfgOutput { cfg: new_cfg, id_map } = builder.finish(cfg);
                    cfg = new_cfg;
                    id_map
                }};
            }

            macro_rules! new_task {
                ($dollar($tt2:tt)*) => {
                    build! { $crate::kernel::CfgTaskBuilder::new(), $dollar($tt2)* }
                };
            }

            macro_rules! new_hunk {
                ([u8], len = $len:expr) => {new_hunk!([u8], len = $len, align = 1)};
                ([u8], len = $len:expr, align = $align:expr) => {
                    call!($crate::kernel::cfg_new_hunk_bytes, $len, $align)
                };
                ($ty:ty) => {call!($crate::kernel::cfg_new_hunk::<_, $ty>)};
            }

            // `$ctx` will be updated by the code generated by `call!`

            let id_map = {
                $($tt)*
            };

            $crate::kernel::CfgOutput { cfg, id_map }
        }
    };

    (
        [$dollar:tt]
        $($tt:tt)*
    ) => {
        compile_error!("invalid syntax")
    };

    ($($tt:tt)*) => {
        $crate::configure! {
            [$]
            $($tt)*
        }
    };
}

/// Attach a configuration function (defined by [`configure!`]) to a "system"
/// type by implementing [`KernelCfg`] on `$sys`.
///
/// [`KernelCfg`]: crate::kernel::KernelCfg
#[macro_export]
macro_rules! build {
    ($sys:ty, $configure:expr) => {{
        use ::core::cell::UnsafeCell;
        use $crate::{
            kernel::{CfgBuilder, HunkAttr, HunkInitAttr, KernelCfg, Port, TaskAttr, TaskCb},
            utils::{AlignedStorage, AssertSendSync, Init},
        };

        // `$configure` produces two values: a `CfgBuilder` and an ID map
        // (custom type). We need the first one to be `const` so that we can
        // calculate the values of generic parameters based on its contents.
        const CFG: CfgBuilder<$sys> = $configure(CfgBuilder::new()).cfg;

        // The second value can be just `let`
        let id_map = $configure(CfgBuilder::new()).id_map;

        // Instantiiate task structures
        $crate::array_item_from_fn! {
            const TASK_ATTR_POOL: [TaskAttr<$sys>; _] =
                (0..CFG.tasks.len()).map(|i| CFG.tasks.get(i).to_attr());
            static TASK_CB_POOL:
                [TaskCb<$sys, <$sys as Port>::PortTaskState>; _] =
                    (0..CFG.tasks.len()).map(|i| CFG.tasks.get(i).to_state(&TASK_ATTR_POOL[i]));
        }

        // Instantiate hunks
        static HUNK_POOL: AssertSendSync<
            UnsafeCell<AlignedStorage<{ CFG.hunk_pool_len }, { CFG.hunk_pool_align }>>,
        > = Init::INIT;
        const HUNK_INITS: [HunkInitAttr; { CFG.hunks.len() }] = CFG.hunks.to_array();

        // Safety: We are `build!`, so it's okay to `impl` this
        unsafe impl KernelCfg for $sys {
            const HUNK_ATTR: HunkAttr = HunkAttr {
                hunk_pool: || HUNK_POOL.0.get() as *const u8,
                inits: &HUNK_INITS,
            };

            #[inline(always)]
            fn task_cb_pool() -> &'static [TaskCb<$sys, <$sys as Port>::PortTaskState>] {
                &TASK_CB_POOL
            }
        }

        id_map
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! array_item_from_fn {
    ($(
        $static_or_const:tt $out:ident: [$ty:ty; _] = (0..$len:expr).map(|$var:ident| $map:expr);
    )*) => {$(
        $static_or_const $out: [$ty; { $len }] = {
            let mut values = [$crate::prelude::Init::INIT; { $len }];
            let mut i = 0;
            while i < $len {
                values[i] = {
                    let $var = i;
                    $map
                };
                i += 1;
            }
            values
        };
    )*};
}

// The "real" public interface ends here
// ---------------------------------------------------------------------------

#[doc(hidden)]
pub struct CfgBuilder<System> {
    _phantom: PhantomData<System>,
    pub hunks: ComptimeVec<super::HunkInitAttr>,
    pub hunk_pool_len: usize,
    pub hunk_pool_align: usize,
    pub tasks: ComptimeVec<CfgBuilderTask<System>>,
}

impl<System> CfgBuilder<System> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
            hunks: ComptimeVec::new(),
            hunk_pool_len: 0,
            hunk_pool_align: 1,
            tasks: ComptimeVec::new(),
        }
    }
}

/// Output of [a configuration function].
///
/// In a configuration function, use `call!` or `build!` to call other
/// configuration functions (i.e., the functions returning this type).
///
/// [a configuration function]: configure
pub struct CfgOutput<System, T> {
    #[doc(hidden)]
    pub cfg: CfgBuilder<System>,

    #[doc(hidden)]
    pub id_map: T,
}

/// Used by `new_hunk!` in configuraton functions
#[doc(hidden)]
pub const fn cfg_new_hunk<System, T: Init>(
    mut cfg: CfgBuilder<System>,
) -> CfgOutput<System, hunk::Hunk<System, T>> {
    let align = mem::align_of::<T>();
    let size = mem::size_of::<T>();

    // Round up `hunk_pool_len`
    cfg.hunk_pool_len = (cfg.hunk_pool_len + align - 1) / align * align;

    let start = cfg.hunk_pool_len;

    cfg.hunks = cfg.hunks.push(hunk::HunkInitAttr {
        offset: start,
        init: |dest| unsafe {
            *(dest as *mut _) = T::INIT;
        },
    });

    cfg.hunk_pool_len += size;
    if align > cfg.hunk_pool_align {
        cfg.hunk_pool_align = align;
    }

    let hunk = unsafe { hunk::Hunk::from_range(start, size) };

    CfgOutput { cfg, id_map: hunk }
}

/// Used by `new_hunk!` in configuraton functions
#[doc(hidden)]
pub const fn cfg_new_hunk_bytes<System>(
    mut cfg: CfgBuilder<System>,
    len: usize,
    align: usize,
) -> CfgOutput<System, hunk::Hunk<System, [u8]>> {
    if !align.is_power_of_two() {
        panic!("`align` is not power of two");
    }

    // Round up `hunk_pool_len`
    cfg.hunk_pool_len = (cfg.hunk_pool_len + align - 1) / align * align;

    // The hunk pool is zero-initialized by default
    let start = cfg.hunk_pool_len;
    let hunk = unsafe { hunk::Hunk::from_range(start, len) };
    cfg.hunk_pool_len += len;
    if align > cfg.hunk_pool_align {
        cfg.hunk_pool_align = align;
    }

    CfgOutput { cfg, id_map: hunk }
}

/// Used by `new_task!` in configuraton functions
#[doc(hidden)]
pub struct CfgTaskBuilder<System> {
    _phantom: PhantomData<System>,
    start: Option<fn(usize)>,
    param: usize,
    stack: Option<TaskStack<System>>,
}

enum TaskStack<System> {
    Auto(usize),
    Hunk(hunk::Hunk<System, [UnsafeCell<u8>]>),
    // TODO: Externally supplied stack? It's blocked by
    //       <https://github.com/rust-lang/const-eval/issues/11>, I think
}

impl<System: Port> CfgTaskBuilder<System> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
            start: None,
            param: 0,
            stack: None,
        }
    }

    pub const fn start(self, start: fn(usize)) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    pub const fn param(self, param: usize) -> Self {
        Self { param, ..self }
    }

    pub const fn stack_size(self, stack_size: usize) -> Self {
        // FIXME: `Option::is_some` is not `const fn` yet
        if let Some(_) = self.stack {
            panic!("the task's stack is already specified");
        }

        Self {
            stack: Some(TaskStack::Auto(stack_size)),
            ..self
        }
    }

    pub const fn stack_hunk(self, stack_hunk: hunk::Hunk<System, [UnsafeCell<u8>]>) -> Self {
        // FIXME: `Option::is_some` is not `const fn` yet
        if let Some(_) = self.stack {
            panic!("the task's stack is already specified");
        }

        Self {
            stack: Some(TaskStack::Hunk(stack_hunk)),
            ..self
        }
    }

    pub const fn finish(
        self,
        mut cfg: CfgBuilder<System>,
    ) -> CfgOutput<System, task::Task<System>> {
        // FIXME: `Option::unwrap_or` is not `const fn` yet
        let stack = if let Some(stack) = self.stack {
            stack
        } else {
            TaskStack::Auto(System::STACK_DEFAULT_SIZE)
        };
        let stack = match stack {
            TaskStack::Auto(size) => {
                let CfgOutput {
                    cfg: new_cfg,
                    id_map: hunk,
                } = cfg_new_hunk_bytes(cfg, size, System::STACK_ALIGN);
                cfg = new_cfg;

                // Safety:
                unsafe { hunk.transmute() }
            }
            TaskStack::Hunk(hunk) => hunk,
        };

        cfg.tasks = cfg.tasks.push(CfgBuilderTask {
            start: if let Some(x) = self.start {
                x
            } else {
                panic!("`start` (task entry point) is not specified")
            },
            param: self.param,
            stack,
        });

        let task = unsafe { task::Task::from_id(NonZeroUsize::new_unchecked(cfg.tasks.len())) };

        CfgOutput { cfg, id_map: task }
    }
}

#[doc(hidden)]
pub struct CfgBuilderTask<System> {
    start: fn(usize),
    param: usize,
    stack: hunk::Hunk<System, [UnsafeCell<u8>]>,
}

impl<System> Clone for CfgBuilderTask<System> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            param: self.param,
            stack: self.stack,
        }
    }
}

impl<System> Copy for CfgBuilderTask<System> {}

impl<System: Port> CfgBuilderTask<System> {
    pub const fn to_state(
        &self,
        attr: &'static task::TaskAttr<System>,
    ) -> task::TaskCb<System, System::PortTaskState> {
        task::TaskCb {
            port_task_state: System::PORT_TASK_STATE_INIT,
            attr,
            _force_int_mut: crate::utils::AssertSendSync(core::cell::UnsafeCell::new(())),
        }
    }

    pub const fn to_attr(&self) -> task::TaskAttr<System> {
        task::TaskAttr {
            entry_point: self.start,
            entry_param: self.param,
            stack: AssertSendSync(self.stack),
        }
    }
}
