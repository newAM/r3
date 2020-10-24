(function() {var implementors = {};
implementors["chrono"] = [{"text":"impl AddAssign&lt;Duration&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Duration&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Duration&gt; for NaiveTime","synthetic":false,"types":[]}];
implementors["constance"] = [{"text":"impl AddAssign&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Duration&gt; for Time","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;T&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;&amp;'a Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;&amp;'a T&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["staticvec"] = [{"text":"impl&lt;const N:&nbsp;usize, '_&gt; AddAssign&lt;&amp;'_ str&gt; for StaticString&lt;N&gt;","synthetic":false,"types":[]}];
implementors["tock_registers"] = [{"text":"impl&lt;T:&nbsp;IntLike, R:&nbsp;RegisterLongName&gt; AddAssign&lt;FieldValue&lt;T, R&gt;&gt; for FieldValue&lt;T, R&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()