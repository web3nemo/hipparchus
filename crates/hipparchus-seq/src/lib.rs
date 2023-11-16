//! ## Create Sequence
//! 
//! Here's an example to create the arithmetic sequence via ``hipparchus``:
//! 
//! ```rust
//! 
//! use hipparchus_seq::Sequence;
//! 
//! let v = Sequence::Arithmetic { init: 1, difference: 1 }.vec(5);
//! 
//! ```
//! 
//! Below is a full list of all sequences ``hipperchus`` has supported:
//! 
//! | Sequence | Syntax | Feature |
//! | :-- | :-- | :-- |
//! | Arithmetic | { init:T, difference:T } | arithmetic sequence with init value and difference |
//! | Geometric | { init:T, ratio:T } | geometric sequence with init value and ratio |
//! | Natural | (bool) | natural sequence starting with 0/1 |
//! | Odd | - | odd sequence starting with 1 |
//! | Even | (bool) | even sequence starting with 0/1 |
//! | Power | (T) | power sequence starting with 1 with radix |
//! | Triangular | - | triangular sequence starting with 1 |
//! | Square | - | square sequence starting with 1 |
//! | Cubic | - | cubic sequence starting with 1 |
//! | Harmonic | { init:T, difference:T } | harmonic sequence with init value and difference |
//! | Fibonacci | - | fibonacci sequence starting with 0, 1 |
//! | Lucas | - | lucas sequence starting with 2, 1 |
//! | Padova | - | padova sequence |
//! | Catalan | - | catalan sequence |
//! | LookAndSay | (usize) | look and say sequence starting with a usize value | 
//! 
//! And hipparchus-mean support recursive, map and fold OPs to generate complicated or derived sequences. 
//! Please refer to codes written in unit tests of sequence module.
//! 

// re-exports
pub use self::sequence::*;

// modules
pub mod sequence;
