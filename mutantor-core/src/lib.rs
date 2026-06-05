//! A procedural mutation testing framework for Rust.
//!
//! This crate provides utilities for generating and evaluating mutants
//! of Rust functions during testing.
//!
//! Mutation testing measures test quality by introducing small changes
//! (mutants) into a program and verifying that existing tests detect
//! the altered behavior.
//!
//! # Features
//!
//! - Arithmetic operator mutations
//! - Conditional operator mutations
//! - Logical operator mutations
//! - Relational operator mutations
//! - Shift operator mutations
//! - Statement deletion mutations
//! - Input parameter mutations
//! - Randomized test input generation
//! - Configurable mutation scoring
//!
//! # Basic Example
//!
//! ```rust,ignore
//! use mutation_testing::generate_mutants;
//!
//! #[generate_mutants(
//!     AOR,
//!     ROR,
//!     IPVR,
//!     acc = 80,
//!     m_count = 2
//! )]
//! fn add(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//! ```
//!
//! During test execution, mutant versions of `add` will be generated
//! and compared against the original implementation.
//!
//! # Modules
//!
//! - [`flag_macros`] — Helper mutation-testing macros.
//! - [`mutation_builder`] — Attribute configuration parsing.
//! - [`mutation_collector`] — Mutant generation and test creation.
//! - [`mutation_operators`] — Supported mutation operators.
//! - [`rand_val`] — Random value generation through the `Mutable` trait.
//!
//! # Re-exports
//!
//! The crate re-exports the `rand` crate for use by generated code.

/// Mutation-testing helper macros.
///
/// Contains marker macros such as:
///
/// - `sdl!`
/// - `ignore!`
pub mod flag_macros;

/// Internal AST mutation visitor implementation.
///
/// Traverses syntax trees and applies mutation operators.
mod mutation;

/// Mutation configuration parser.
///
/// Parses arguments supplied to the mutation-generation attribute
/// macro.
pub mod mutation_builder;

/// Mutant generation and mutation-score calculation.
///
/// Responsible for generating mutated functions and corresponding
/// test modules.
pub mod mutation_collector;

/// Input mutation generation.
///
/// Generates mutated function arguments during mutant execution.
mod mutation_input;

/// Supported mutation operators.
///
/// Defines the [`MutationOperators`] enum and parsing utilities.
pub mod mutation_operators;

/// Random value generation utilities.
///
/// Provides the [`Mutable`] trait and implementations for common Rust
/// types.
pub mod rand_val;

/// Re-export of the `rand` crate.
///
/// Generated mutation code relies on `rand`, so it is re-exported to
/// ensure compatibility with generated test code.
pub use rand;