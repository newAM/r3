(function() {var implementors = {};
implementors["constance"] = [{"text":"impl From&lt;Result&lt;(), BadContextError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), BadIdError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), BadParamError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), BadObjectStateError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), ActivateTaskError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;ActivateTaskError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), GetCurrentTaskError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;GetCurrentTaskError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), InterruptTaskError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;InterruptTaskError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), SetTaskPriorityError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;SetTaskPriorityError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), ExitTaskError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;ExitTaskError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), CpuLockError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;CpuLockError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), BoostPriorityError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;BoostPriorityError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), TimeError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;TimeError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), AdjustTimeError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;AdjustTimeError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), WaitError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), WaitTimeoutError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitTimeoutError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitError&gt; for WaitTimeoutError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), ParkError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;ParkError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitError&gt; for ParkError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), ParkTimeoutError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;ParkTimeoutError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitTimeoutError&gt; for ParkTimeoutError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), UnparkError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;UnparkError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), UnparkExactError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;UnparkExactError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), SleepError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;SleepError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), UpdateEventGroupError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;UpdateEventGroupError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), GetEventGroupError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;GetEventGroupError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), PollEventGroupError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;PollEventGroupError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), WaitEventGroupError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitEventGroupError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitError&gt; for WaitEventGroupError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), WaitEventGroupTimeoutError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitEventGroupTimeoutError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitTimeoutError&gt; for WaitEventGroupTimeoutError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), GetSemaphoreError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;GetSemaphoreError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), DrainSemaphoreError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;DrainSemaphoreError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), SignalSemaphoreError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;SignalSemaphoreError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), PollSemaphoreError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;PollSemaphoreError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), WaitSemaphoreError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitSemaphoreError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitError&gt; for WaitSemaphoreError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), WaitSemaphoreTimeoutError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitSemaphoreTimeoutError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;WaitTimeoutError&gt; for WaitSemaphoreTimeoutError","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), SetInterruptLinePriorityError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;SetInterruptLinePriorityError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), EnableInterruptLineError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;EnableInterruptLineError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), PendInterruptLineError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;PendInterruptLineError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), ClearInterruptLineError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;ClearInterruptLineError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), QueryInterruptLineError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;QueryInterruptLineError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), StartTimerError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;StartTimerError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), StopTimerError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;StopTimerError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), SetTimerDelayError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;SetTimerDelayError&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;Result&lt;(), SetTimerPeriodError&gt;&gt; for ResultCode","synthetic":false,"types":[]},{"text":"impl From&lt;SetTimerPeriodError&gt; for ResultCode","synthetic":false,"types":[]}];
implementors["env_logger"] = [{"text":"impl&lt;'a, T&gt; From&lt;T&gt; for Env&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;Cow&lt;'a, str&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["errno"] = [{"text":"impl From&lt;Errno&gt; for Error","synthetic":false,"types":[]}];
implementors["humantime"] = [{"text":"impl From&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl From&lt;SystemTime&gt; for Timestamp","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T&gt; From&lt;T&gt; for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;(T, T)&gt; for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["once_cell"] = [{"text":"impl&lt;T&gt; From&lt;T&gt; for OnceCell&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for OnceCell&lt;T&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl From&lt;Span&gt; for Span","synthetic":false,"types":[]},{"text":"impl From&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl From&lt;Group&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Punct&gt; for TokenTree","synthetic":false,"types":[]},{"text":"impl From&lt;Literal&gt; for TokenTree","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl&lt;X:&nbsp;SampleUniform&gt; From&lt;Range&lt;X&gt;&gt; for Uniform&lt;X&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;ChaChaCore&gt; for ChaChaRng","synthetic":false,"types":[]},{"text":"impl From&lt;TimerError&gt; for Error","synthetic":false,"types":[]}];
implementors["regex"] = [{"text":"impl&lt;'t&gt; From&lt;Match&lt;'t&gt;&gt; for Range&lt;usize&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'t&gt; From&lt;Match&lt;'t&gt;&gt; for &amp;'t str","synthetic":false,"types":[]},{"text":"impl&lt;'t&gt; From&lt;Match&lt;'t&gt;&gt; for Range&lt;usize&gt;","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["staticvec"] = [{"text":"impl&lt;T:&nbsp;Ord, const N1:&nbsp;usize, const N2:&nbsp;usize&gt; From&lt;StaticVec&lt;T, N1&gt;&gt; for StaticHeap&lt;T, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord, const N:&nbsp;usize&gt; From&lt;StaticVec&lt;T, N&gt;&gt; for StaticHeap&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord, const N1:&nbsp;usize, const N2:&nbsp;usize&gt; From&lt;[T; N1]&gt; for StaticHeap&lt;T, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord, const N:&nbsp;usize&gt; From&lt;[T; N]&gt; for StaticHeap&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;DecodeUtf16Error&gt; for StringError","synthetic":false,"types":[]},{"text":"impl From&lt;Utf8Error&gt; for StringError","synthetic":false,"types":[]},{"text":"impl&lt;const N:&nbsp;usize&gt; From&lt;CapacityError&lt;N&gt;&gt; for StringError","synthetic":false,"types":[]},{"text":"impl&lt;'a, const N:&nbsp;usize&gt; From&lt;&amp;'a str&gt; for StaticString&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;const N1:&nbsp;usize, const N2:&nbsp;usize&gt; From&lt;StaticVec&lt;u8, N1&gt;&gt; for StaticString&lt;N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;const N:&nbsp;usize&gt; From&lt;StaticVec&lt;u8, N&gt;&gt; for StaticString&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy, const N:&nbsp;usize, '_&gt; From&lt;&amp;'_ [T]&gt; for StaticVec&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy, const N:&nbsp;usize, '_&gt; From&lt;&amp;'_ mut [T]&gt; for StaticVec&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, const N1:&nbsp;usize, const N2:&nbsp;usize&gt; From&lt;[T; N1]&gt; for StaticVec&lt;T, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, const N:&nbsp;usize&gt; From&lt;[T; N]&gt; for StaticVec&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy, const N1:&nbsp;usize, const N2:&nbsp;usize, '_&gt; From&lt;&amp;'_ [T; N1]&gt; for StaticVec&lt;T, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy, const N:&nbsp;usize, '_&gt; From&lt;&amp;'_ [T; N]&gt; for StaticVec&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy, const N1:&nbsp;usize, const N2:&nbsp;usize, '_&gt; From&lt;&amp;'_ mut [T; N1]&gt; for StaticVec&lt;T, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy, const N:&nbsp;usize, '_&gt; From&lt;&amp;'_ mut [T; N]&gt; for StaticVec&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, const N1:&nbsp;usize, const N2:&nbsp;usize&gt; From&lt;StaticHeap&lt;T, N1&gt;&gt; for StaticVec&lt;T, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, const N:&nbsp;usize&gt; From&lt;StaticHeap&lt;T, N&gt;&gt; for StaticVec&lt;T, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;const N1:&nbsp;usize, const N2:&nbsp;usize&gt; From&lt;StaticString&lt;N1&gt;&gt; for StaticVec&lt;u8, N2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;const N:&nbsp;usize&gt; From&lt;StaticString&lt;N&gt;&gt; for StaticVec&lt;u8, N&gt;","synthetic":false,"types":[]}];
implementors["svg"] = [{"text":"impl From&lt;Value&gt; for String","synthetic":false,"types":[]},{"text":"impl From&lt;i8&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;i16&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;i32&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;i64&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;isize&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;u8&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;u16&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;u32&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;usize&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;f32&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;f64&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;String&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;bool&gt; for Value","synthetic":false,"types":[]},{"text":"impl&lt;'l&gt; From&lt;&amp;'l str&gt; for Value","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;Vec&lt;T&gt;&gt; for Value <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;Value&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1&gt; From&lt;(T0, T1)&gt; for Value <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Value&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Value&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2, T3&gt; From&lt;(T0, T1, T2, T3)&gt; for Value <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Value&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Value&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Value&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T3: Into&lt;Value&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl From&lt;Command&gt; for String","synthetic":false,"types":[]},{"text":"impl From&lt;Vec&lt;Command&gt;&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;Data&gt; for Vec&lt;Command&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;Data&gt; for Value","synthetic":false,"types":[]},{"text":"impl From&lt;Parameters&gt; for String","synthetic":false,"types":[]},{"text":"impl From&lt;Vec&lt;f32&gt;&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;Parameters&gt; for Vec&lt;Number&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;i8&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;i16&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;i32&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;i64&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;isize&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;u8&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;u16&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;u32&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;usize&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;f32&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl From&lt;f64&gt; for Parameters","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1&gt; From&lt;(T0, T1)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2&gt; From&lt;(T0, T1, T2)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2, T3&gt; From&lt;(T0, T1, T2, T3)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T3: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2, T3, T4&gt; From&lt;(T0, T1, T2, T3, T4)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T3: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T4: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2, T3, T4, T5&gt; From&lt;(T0, T1, T2, T3, T4, T5)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T3: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T4: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T5: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2, T3, T4, T5, T6&gt; From&lt;(T0, T1, T2, T3, T4, T5, T6)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T3: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T4: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T5: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T6: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T0, T1, T2, T3, T4, T5, T6, T7&gt; From&lt;(T0, T1, T2, T3, T4, T5, T6, T7)&gt; for Parameters <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T0: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T1: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T2: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T3: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T4: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T5: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T6: Into&lt;Parameters&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T7: Into&lt;Parameters&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl From&lt;SelfValue&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;SelfType&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Super&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Crate&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Extern&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Underscore&gt; for Ident","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;MetaList&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;MetaNameValue&gt; for Meta","synthetic":false,"types":[]},{"text":"impl From&lt;Meta&gt; for NestedMeta","synthetic":false,"types":[]},{"text":"impl From&lt;Lit&gt; for NestedMeta","synthetic":false,"types":[]},{"text":"impl From&lt;FieldsNamed&gt; for Fields","synthetic":false,"types":[]},{"text":"impl From&lt;FieldsUnnamed&gt; for Fields","synthetic":false,"types":[]},{"text":"impl From&lt;VisPublic&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;VisCrate&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;VisRestricted&gt; for Visibility","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBox&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprInPlace&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprArray&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprCall&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMethodCall&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTuple&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBinary&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprUnary&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLit&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprCast&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprType&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLet&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprIf&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprWhile&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprForLoop&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprLoop&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMatch&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprClosure&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprUnsafe&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBlock&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAssign&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAssignOp&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprField&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprIndex&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprRange&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprPath&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprReference&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprBreak&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprContinue&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprReturn&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprMacro&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprStruct&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprRepeat&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprParen&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprGroup&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTry&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprAsync&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprTryBlock&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprYield&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;ExprVerbatim&gt; for Expr","synthetic":false,"types":[]},{"text":"impl From&lt;usize&gt; for Index","synthetic":false,"types":[]},{"text":"impl From&lt;PatWild&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatIdent&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatStruct&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatTupleStruct&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatPath&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatTuple&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatBox&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatRef&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatLit&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatRange&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatSlice&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatMacro&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;PatVerbatim&gt; for Pat","synthetic":false,"types":[]},{"text":"impl From&lt;TypeParam&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;LifetimeDef&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;ConstParam&gt; for GenericParam","synthetic":false,"types":[]},{"text":"impl From&lt;Ident&gt; for TypeParam","synthetic":false,"types":[]},{"text":"impl From&lt;TraitBound&gt; for TypeParamBound","synthetic":false,"types":[]},{"text":"impl From&lt;Lifetime&gt; for TypeParamBound","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateType&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateLifetime&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;PredicateEq&gt; for WherePredicate","synthetic":false,"types":[]},{"text":"impl From&lt;ItemExternCrate&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemUse&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemStatic&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemConst&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemFn&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemMod&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemForeignMod&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemType&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemExistential&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemStruct&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemEnum&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemUnion&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemTrait&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemTraitAlias&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemImpl&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemMacro&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemMacro2&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;ItemVerbatim&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;DeriveInput&gt; for Item","synthetic":false,"types":[]},{"text":"impl From&lt;UsePath&gt; for UseTree","synthetic":false,"types":[]},{"text":"impl From&lt;UseName&gt; for UseTree","synthetic":false,"types":[]},{"text":"impl From&lt;UseRename&gt; for UseTree","synthetic":false,"types":[]},{"text":"impl From&lt;UseGlob&gt; for UseTree","synthetic":false,"types":[]},{"text":"impl From&lt;UseGroup&gt; for UseTree","synthetic":false,"types":[]},{"text":"impl From&lt;ForeignItemFn&gt; for ForeignItem","synthetic":false,"types":[]},{"text":"impl From&lt;ForeignItemStatic&gt; for ForeignItem","synthetic":false,"types":[]},{"text":"impl From&lt;ForeignItemType&gt; for ForeignItem","synthetic":false,"types":[]},{"text":"impl From&lt;ForeignItemMacro&gt; for ForeignItem","synthetic":false,"types":[]},{"text":"impl From&lt;ForeignItemVerbatim&gt; for ForeignItem","synthetic":false,"types":[]},{"text":"impl From&lt;TraitItemConst&gt; for TraitItem","synthetic":false,"types":[]},{"text":"impl From&lt;TraitItemMethod&gt; for TraitItem","synthetic":false,"types":[]},{"text":"impl From&lt;TraitItemType&gt; for TraitItem","synthetic":false,"types":[]},{"text":"impl From&lt;TraitItemMacro&gt; for TraitItem","synthetic":false,"types":[]},{"text":"impl From&lt;TraitItemVerbatim&gt; for TraitItem","synthetic":false,"types":[]},{"text":"impl From&lt;ImplItemConst&gt; for ImplItem","synthetic":false,"types":[]},{"text":"impl From&lt;ImplItemMethod&gt; for ImplItem","synthetic":false,"types":[]},{"text":"impl From&lt;ImplItemType&gt; for ImplItem","synthetic":false,"types":[]},{"text":"impl From&lt;ImplItemExistential&gt; for ImplItem","synthetic":false,"types":[]},{"text":"impl From&lt;ImplItemMacro&gt; for ImplItem","synthetic":false,"types":[]},{"text":"impl From&lt;ImplItemVerbatim&gt; for ImplItem","synthetic":false,"types":[]},{"text":"impl From&lt;ArgSelfRef&gt; for FnArg","synthetic":false,"types":[]},{"text":"impl From&lt;ArgSelf&gt; for FnArg","synthetic":false,"types":[]},{"text":"impl From&lt;ArgCaptured&gt; for FnArg","synthetic":false,"types":[]},{"text":"impl From&lt;Pat&gt; for FnArg","synthetic":false,"types":[]},{"text":"impl From&lt;Type&gt; for FnArg","synthetic":false,"types":[]},{"text":"impl From&lt;LitStr&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitByteStr&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitByte&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitChar&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitInt&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitFloat&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitBool&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;LitVerbatim&gt; for Lit","synthetic":false,"types":[]},{"text":"impl From&lt;DataStruct&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;DataEnum&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;DataUnion&gt; for Data","synthetic":false,"types":[]},{"text":"impl From&lt;TypeSlice&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeArray&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypePtr&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeReference&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeBareFn&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeNever&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeTuple&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypePath&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeTraitObject&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeImplTrait&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeParen&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeGroup&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeInfer&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeMacro&gt; for Type","synthetic":false,"types":[]},{"text":"impl From&lt;TypeVerbatim&gt; for Type","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Path <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;PathSegment&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for PathSegment <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;Ident&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl From&lt;LexError&gt; for Error","synthetic":false,"types":[]}];
implementors["tock_registers"] = [{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;LocalRegisterCopy&lt;u8, R&gt;&gt; for u8","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;LocalRegisterCopy&lt;u16, R&gt;&gt; for u16","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;LocalRegisterCopy&lt;u32, R&gt;&gt; for u32","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;LocalRegisterCopy&lt;u64, R&gt;&gt; for u64","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;FieldValue&lt;u8, R&gt;&gt; for u8","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;FieldValue&lt;u16, R&gt;&gt; for u16","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;FieldValue&lt;u32, R&gt;&gt; for u32","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;RegisterLongName&gt; From&lt;FieldValue&lt;u64, R&gt;&gt; for u64","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()