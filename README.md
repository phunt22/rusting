# Rusting

This repo is a collection of algorithm exercises in Rust, inspired by Rustlings and intended to be extra practice for CSE421 at UW.

Please abide by all academic integrity policies, see the disclaimer below for more information.

## Prerequisites

- Read the Rust book and complete Rustlings
- Basic data structures and algorithms knowledge (CS foundations)

## Setup

1. **Clone the repository**

   ```bash
   git clone <repo-url>
   cd rusting/algos
   ```

2. **Run tests for a specific algorithm <ALGO>**

   ```bash
   cargo ex <gale_shapley>  # or: cargo test --test gale_shapley
   ```

3. **Run all algorithm tests**

   ```bash
   cargo ex-all  # or: cargo test
   ```

4. **Implement algorithms**
   - Find the algorithm stub in `src/<algorithm>.rs`
   - Replace the `todo!("Your implementation here")` with your implementation
   - If tests pass, you are good!

## Algorithms

- **Gale-Shapley** (`src/gale_shapley.rs`) - Stable matching algorithm

  - Run: `cargo ex gale_shapley`

- **More coming soon**

## Disclaimer

This repository is stricly intended for extra practice in CSE421 and learning the Rust programming language. Please ensure that you are following the University of Washington's (and specifically CSE421's) Academic Integrity policies. Do not post/commit any solutions!

## Contributing

If you have any issues or would like to add any additional problems, feel free to make a new branch and open a PR. If you have questions, etc. you can reach me at [phunt22@uw.edu](mailto:phunt22@uw.edu). Thanks!

### Adding New Algorithms

Each algorithm follows this pattern:

1. Add module declaration to `src/lib.rs`
2. Create `src/<algorithm>.rs` with function stub, comment, and todo!() in the method body
3. Create `tests/<algorithm>.rs` with acceptance tests
4. Update the "Algorithms" section in this README

- Tests focus on correctness properties, not specific outputs and are not intended to be completely exhaustive.

Thanks, hope that this repo is of some help!
