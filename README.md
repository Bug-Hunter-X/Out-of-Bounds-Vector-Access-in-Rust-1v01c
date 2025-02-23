# Out of Bounds Vector Access in Rust
This example demonstrates a common error in Rust: accessing an element beyond the bounds of a vector using the `get()` method.

The code attempts to access the element at index 10 of a vector containing only 5 elements.  While this will not cause a runtime panic in Rust (unlike some other languages), it returns `None` instead of the expected value or panicking.  This can lead to subtle runtime errors if not properly handled. The solution demonstrates how to handle the `None` case gracefully.
