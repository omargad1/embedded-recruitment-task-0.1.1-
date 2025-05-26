# Embedded Rust Server - Multithreaded Refactor

##  Overview

This repository contains my completed solution Embedded Rust Server  which involved debugging and refactoring a single-threaded Rust server into a **robust, multithreaded system**. The goal was to enable **concurrent client handling**, ensure **thread safety**, and optimize for **performance and scalability**.

---

##  What Was Done

###  Original Issues Fixed

- Identified and resolved multiple **intentional bugs** in the original single-threaded implementation.
- Addressed logic issues, improper state management, and faulty message handling.

###  Transition to Multithreading

- Replaced single-threaded handling with **Rust threads (`std::thread`)**.
- Implemented **mutexes (`Arc<Mutex<_>>`)**  for **safe state sharing**.
- Ensured **data consistency** across threads and clients.

###  Architectural Improvements

- Separated responsibilities into modular components (socket handling, state, message processing).
- Improved error handling and **log verbosity** to aid in debugging and production readiness.
- Optimized code for maintainability and future extensions.

---

##  Testing & Validation

###  Test Suite

- Ran all **provided test cases** from `tests/client_test.rs` using:

  ```bash
  cargo test

### **Repository Structure**
```plaintext
.
|── proto/
│   └── messages.proto        # IDL with messages server handle
├── src/
│   ├── main.rs               # Server implementation (single-threaded and buggy)
│   └── lib.rs                # Core server logic
├── tests/
│   └── client_test.rs        # Client test suite
├── .gitignore
├── build.rs                  # Build script for compiling the Proto file
├── Cargo.toml                # Rust dependencies and configuration
├── README.md                 # Task instructions
└── SOLUTION.md               # Place for your findings and analysis
```

## Running Tests

To run the provided test suite:

```bash
cargo test
```







