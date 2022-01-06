string types in Rust
CStr, str, Slice, CString, OsString, OsStr and Owned type

To get command line arguments
std::env::args_os or std::env::args

Rust doesn’t guarantee TOC (Tail Call Optimization).

&self : when Read-only reference is required to the function.
self : When a value is to be consumed by the function.
&mut : When a value needs to be mutated by the function with consuming it.

Rust Error Handling is categorized into three parts:

1. Recoverable Error with Results :
   - If an error occurs, the program doesn’t stop completely.
   - Instead, it can easily be interpreted or responded.
2. Unrecoverable Errors with Panic :
   - If something wrong goes with the code, Rust’s panic macro comes into action, shows the error message, clean the error and then quit.
3. Panic or Not to Panic :
   - When you are dicey about calling panic or not, write the code that panics and the process will continue as 2nd.
