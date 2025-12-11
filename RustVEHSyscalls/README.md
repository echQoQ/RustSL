# RustVEHSyscalls

**RustVEHSyscalls** is a Rust-based port of the [LayeredSyscall](https://github.com/WKL-Sec/LayeredSyscall) project. This tool performs indirect syscalls while generating legitimate API call stack frames by abusing Vectored Exception Handling (VEH) to bypass user-land EDR hooks in Windows.

## How It Works

**RustVEHSyscalls** performs indirect syscalls by abusing Vectored Exception Handling (VEH) to generate legitimate API call stack frames. By calling a standard Windows API function and setting a hardware breakpoint within it, the function's call stack is captured. This breakpoint then lets VEH redirect the process to a syscall wrapper in `ntdll.dll`, preserving the original API's call stack structure. This approach enables syscalls to appear as if they originate from legitimate Windows API calls.

### Setup and Cleanup Functions

**RustVEHSyscalls** provides functions to initialize and clean up the Vectored Exception Handling environment necessary for syscall interception. These functions establish the hooks needed to capture and handle indirect syscalls, ensuring clean operation and teardown.

1. **`initialize_hooks()`**:
   - Sets up two vectored exception handlers for adding and managing hardware breakpoints in the system call path. This function allocates memory for the CPU context and retrieves `ntdll.dll`'s base and end addresses for tracing purposes.
2. **`destroy_hooks()`**:
   - Cleans up by removing the added vectored exception handlers.

### Syscall Wrapper

**RustVEHSyscalls** provides a `syscall!` macro that wraps several key steps:

1. **Resolving the Syscall Address and SSN**: The macro uses the **PEB** to locate `ntdll.dll` and parses its **Exception Directory** and **Export Address Table** to retrieve both the syscall’s address and System Service Number (SSN).
2. **Setting Hardware Breakpoint**: Once the syscall address and SSN are resolved, the macro sets a hardware breakpoint, allowing RustVEHSyscalls to intercept the syscall execution.
3. **Invoking the Syscall**: Finally, the macro invokes the syscall with the specified parameters, completing the indirect syscall path.

## Usage

To initialize syscall interception, call `initialize_hooks()` at the start of your `main` function and `destroy_hooks()` to clean up once you're done. You can also adjust the legitimate call stack by modifying the `demofunction()` in the `hooks.rs` module.

```rust
/// Example function designed to maintain a clean call stack.
/// This function can be modified to call different legitimate Windows APIs.
pub unsafe extern "C" fn demofunction() {
    MessageBoxA(null_mut(), null_mut(), null_mut(), 0);
}
```

### Example: Calling `NtCreateUserProcess`

The following example demonstrates how to invoke the `NtCreateUserProcess` syscall. Full test code is available in `lib.rs`.

```rust
fn main() {
    initialize_hooks(); // Set up necessary hooks

    // Initialize all necessary parameters here...

    // Call NtCreateUserProcess syscall
    let status = syscall!(
        "NtCreateUserProcess",
        OrgNtCreateUserProcess,
        &mut process_handle,
        &mut thread_handle,
        desired_access,
        desired_access,
        null_mut(),
        null_mut(),
        0,
        0,
        process_parameters,
        &mut create_info,
        attribute_list
    );

    destroy_hooks(); // Clean up hooks when done
}
```

## Disclaimer

This project is intended **for educational and research purposes only**. Use it responsibly, as any misuse is solely your responsibility—not mine! Always follow ethical guidelines and legal frameworks when doing security research (and, you know, just in general).

## Credits

Special thanks to:

- [LayeredSyscall by White Knight Labs](https://github.com/WKL-Sec/LayeredSyscall) for their work.
- [Resolving System Service Numbers Using The Exception Directory by MDsec](https://www.mdsec.co.uk/2022/04/resolving-system-service-numbers-using-the-exception-directory/) for their insights on resolving SSNs.

## Contributing

Contributions are welcome! If you want to help improve `RustVEHSyscalls` or report bugs, feel free to open an issue or a pull request in the repository.

---
