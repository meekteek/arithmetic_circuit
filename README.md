# Arithmetic Circuit
arithmetic_circuit is a rust library designed to offer a straightforward simple representation of polynomial functions in a computational graph structure.

## Overview
 Its primary purpose is as a learning tool for understanding how zk circuits may behave without any of the cryptographic aspects normally 
 found in zk circuit implementations such as fields, witnesses, prover-verifier model, etc.
#### ✅ TODO's
* [ ] Add prime field for nodes to live in. Make graph generic in a prime field.
* [ ] Add support for parallelization for filling the graph.
* [ ] Add Asynchronous hints.
* [ ] Add more efficient evaluation of the graph.
* [ ] Add support for better graph visualization. More specifically after entire graph is filled with inputs and constants to log final output.

### Logging
This library uses the env_logger crate to provide basic logging information. You may enable this by setting the RUST_LOG level before running your program. The following logging levels are used:
- **Info**: Provides general information about the graph's state.
 <br> RUST_LOG=info
 - **Debug**: Provides more information regarding graph's state that may be useful for debugging.
 <br> RUST_LOG=debug

### Documentation 
To look at the documentation pertaining only to this repo, run 'cargo doc --no-deps --open' 

 ## Note
This library is designed for educational purposes. Since this library abstracts away cryptographic aspects
and provides only basic functionality, it can't be used in any real purpose schemes.
