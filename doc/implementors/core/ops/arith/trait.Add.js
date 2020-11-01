(function() {var implementors = {};
implementors["chrono"] = [{"text":"impl Add&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl Add&lt;FixedOffset&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl Add&lt;FixedOffset&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Add&lt;FixedOffset&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl Add&lt;Duration&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl Add&lt;Duration&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl Add&lt;Duration&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Add&lt;Duration&gt; for Date&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Add&lt;Duration&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;'a, 'b, T:&nbsp;Clone + Integer&gt; Add&lt;&amp;'b Ratio&lt;T&gt;&gt; for &amp;'a Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b, T:&nbsp;Clone + Integer&gt; Add&lt;&amp;'b T&gt; for &amp;'a Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Add&lt;Ratio&lt;T&gt;&gt; for &amp;'a Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Add&lt;T&gt; for &amp;'a Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Add&lt;&amp;'a Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Add&lt;&amp;'a T&gt; for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; Add&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; Add&lt;T&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["pom"] = [{"text":"impl&lt;'b, 'a: 'b, I:&nbsp;Copy + 'static, O:&nbsp;'static, U:&nbsp;'static&gt; Add&lt;Parser&lt;'b, I, U&gt;&gt; for Parser&lt;'a, I, O&gt;","synthetic":false,"types":[]}];
implementors["r3"] = [{"text":"impl Add&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl Add&lt;Duration&gt; for Time","synthetic":false,"types":[]}];
implementors["staticvec"] = [{"text":"impl&lt;const N:&nbsp;usize, '_&gt; Add&lt;&amp;'_ str&gt; for StaticString&lt;N&gt;","synthetic":false,"types":[]}];
implementors["tock_registers"] = [{"text":"impl&lt;T:&nbsp;IntLike, R:&nbsp;RegisterLongName&gt; Add&lt;FieldValue&lt;T, R&gt;&gt; for FieldValue&lt;T, R&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()