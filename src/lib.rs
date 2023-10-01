//! # Arithmetic Circuit
//!
//! `arithmetic_circuit` is a library designed to provide a simple representation of polynomial functions in a 'computational graph'.
//!
//! ## Overview
//! Its primary purpose is as a learning tool for understanding how zk circuits may behave without any of the cryptographic aspects normally
//! found in zk circuit implementations such as fields, witnesses, prover-verifier model, etc.
//!
//! ## TODO's
//! - [ ] Add prime field for nodes to live in. Make graph generic in a prime field
//! - [ ] Add support for parallelization for filling the graph
//! - [ ] Add Asynchronous hints
//! - [ ] Add more efficient evaluation of the graph
//! - [ ] Add support for better graph visualization. More specifically after entire graph is filled with inputs and constants to log final output
//!
//!
//! ## Core Concepts
//!
//! - **Builder**: center of the library. Builder provides methods to define polynomial functions by creating a "graph" of nodes,
//!    handles arithmetic operations in circuit, and asserts + verifies constraints.
//!
//! - **Node**: Represents a fundamental unit or variable in the circuit. Nodes can have actual values or unevaluated expressions
//!   to be resolved at a later time once inputs are given.
//!
//! - **Constraints**: Constraints ensure the validity of the operations performed on the nodes. These can also be thought of as gates.
//!   These are defined automatically when nodes undergo arithmetic operations and can also be manually asserted.
//!
//! ## Getting Started
//!
//! To create a circuit, start with the `Builder`:
//!
//! ```rust
//! use arithmetic_circuit::Builder;
//! let mut builder = Builder::new();
//! let x = builder.init();
//! let y = builder.constant(5);
//! let result = builder.add(x, y);
//! ```
//!
//!
//! ## Logging
//!
//! This library uses env_logger crate to provide basic logging information.
//! You may enable this by setting the RUST_LOG level before running your program.
//! The following logging levels are used:
//!
//! - **Info**: Provides general information about the graph's state.
//! <br> RUST_LOG=info
//! - **Debug**: Provides more information regarding graph's state that may be useful for debugging.
//! <br> RUST_LOG=debug
//!
//!
//! ## Note
//!
//!   This library is designed for educational purposes. Since this library abstracts away cryptographic aspects
//!   and provides only basic functionality, it can't be used in any real purpose schemes.
//!
//!
//!
pub mod builder;
pub mod enums;
pub mod node;
pub use builder::Builder;
pub use node::Node;
