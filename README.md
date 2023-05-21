# leak
A hypersimple "safe" Rust memory leaking program. For educational purposes.

This is meant to demonstrate the absurdity of how Rust considers memory leaks safe.\
There are reasons why `Box::leak` is considered safe, and it technically does not violate\
memory safety (all pointers are still valid), but it is still offputting.
