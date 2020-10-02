initSidebarItems({"enum":[["StringError","This enum represents several different possible \"error states\" that may be encountered while using a `StaticString`."]],"macro":[["sortedstaticvec","Accepts an array of any primitive `Copy` type that has a `PartialOrd` implementation, sorts it, and creates a new `StaticVec` instance from the result in a fully const context compatible manner."],["staticstring","Creates a new `StaticString` from an `&str` literal. This macro can be used in const contexts, in keeping with the other ones in this crate."],["staticvec","Creates a new `StaticVec` from a `vec!`-style pseudo-slice. The newly created `StaticVec` will have a capacity and length exactly equal to the number of elements in the so-called slice. The \"array-like\" `[value; N]` syntax is also supported, and both forms can be used in const contexts. This macro has no particular trait impl requirements for the input type."]],"struct":[["CapacityError","This error indicates that an operation was attempted that would have increased the `length` value of a `StaticVec`, but the `StaticVec` was already at its maximum capacity of `N`."],["PushCapacityError","This error indicates that a push was attempted into a `StaticVec` which failed because the `StaticVec` was already at maximum capacity. It contains the value that failed to be pushed so that it can be reused if needed."],["StaticHeap","A priority queue implemented as a binary heap, built around an instance of `StaticVec<T, N>`."],["StaticHeapDrainSorted","A sorted \"draining\" iterator over the elements of a `StaticHeap`."],["StaticHeapIntoIterSorted","A sorted \"consuming\" iterator over the elements of a `StaticHeap`."],["StaticHeapPeekMut","A struct wrapping a mutable reference to the greatest (or \"maximal\") item in a `StaticHeap`."],["StaticString","A fixed-capacity `String`-like struct built around an instance of `StaticVec<u8, N>`."],["StaticVec","A `Vec`-like struct (mostly directly API-compatible where it can be) implemented with const generics around an array of fixed `N` capacity."],["StaticVecDrain","A \"draining\" iterator, analogous to `vec::Drain`. Instances of `StaticVecDrain` are created by the `drain_iter` method on `StaticVec`, as while the `drain` method does have a similar purpose, it works by immediately returning a new `StaticVec` as opposed to an iterator."],["StaticVecIntoIter","A \"consuming\" iterator that reads each element out of a source [`StaticVec`] by value."],["StaticVecIterConst","Similar to `Iter`, but specifically implemented with [`StaticVec`]s in mind."],["StaticVecIterMut","Similar to `IterMut`, but specifically implemented with [`StaticVec`]s in mind."],["StaticVecSplice","A \"splicing\" iterator, analogous to `vec::Splice`. Instances of `StaticVecSplice` are created by the `splice` method on `StaticVec`."]]});