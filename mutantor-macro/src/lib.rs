//! AI-assisted mutation planning.
//!
//! This module analyzes Rust functions, extracts structural metrics,
//! and uses a Large Language Model (LLM) to automatically generate a
//! mutation-testing strategy.
//!
//! The generated strategy is returned as a [`MutationPlan`] and can be
//! used by the mutation engine to configure:
//!
//! - Mutation operators
//! - Mutation count
//! - Mutation combinations
//! - Mutation probability
//! - Required mutation score
//!
//! # Workflow
//!
//! ```text
//! Rust Function
//!       │
//!       ▼
//! CodeFeatures
//!       │
//!       ▼
//! Prompt Generation
//!       │
//!       ▼
//! OpenRouter API
//!       │
//!       ▼
//! MutationPlan
//!       │
//!       ▼
//! Mutation Test Generation
//! ```
//!
//! # Configuration
//!
//! API credentials are loaded from:
//!
//! ```text
//! mutant.toml
//! ```
//!
//! Example:
//!
//! ```toml
//! api_key = "your-openrouter-api-key"
//! ```
mod ai;

use mutantor_core::mutation_builder::MutationBuilder;
use mutantor_core::mutation_collector:: mutation_collector;
use quote::quote;
use std::fs;
use std::process::Command;
use std::str::FromStr;
use syn::{ItemFn, parse_macro_input};
use mutantor_core::mutation_operators::MutationOperators;
use crate::ai::{ask_llm, build_prompt, CodeFeatures};

/// Generates mutation tests using an explicitly configured
/// mutation strategy.
///
/// See [`MutationBuilder`] for supported configuration options.
#[proc_macro_attribute]
pub fn generate_mutants(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let data = parse_macro_input!(attr as MutationBuilder);

    let func = parse_macro_input!(item as ItemFn);

    let test = mutation_collector(&data, &func);

    let out = quote! {
        #func
        #test
    };


    if let Some(path) = data.path {
        fs::write(&path, out.to_string().as_str()).expect("cannot write file");
        Command::new("rustfmt")
            .arg(path)
            .status().expect("cannot format file");
    };

    out.into()
}
/// Generates mutation tests using an AI-generated mutation plan.
///
/// The target function is analyzed and a language model is used
/// to determine mutation operators, mutation counts, mutation
/// probabilities, and score thresholds.
#[proc_macro_attribute]
pub fn generate_mutants_ai(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut data = parse_macro_input!(attr as MutationBuilder);

    let func = parse_macro_input!(item as ItemFn);

    let features = CodeFeatures::new(&func);

    let prompt = build_prompt(&features);

    let plan = ask_llm(prompt);

    data.mutation_chance = plan.mutation_chance;
    data.acceptable_score = plan.acceptable_score;
    data.mutation_count = plan.mutation_count;
    data.combination_count = plan.combination_count;
    data.operators = plan.operators.iter().map(|x| MutationOperators::from_str(x.as_str()).expect("Bad AI Input")).collect();

    let test = mutation_collector(&data, &func);

    let out = quote! {
        #func
        #test
    };

    if let Some(path) = data.path {
        fs::write(&path, out.to_string().as_str()).expect("cannot write file");
        Command::new("rustfmt")
            .arg(path)
            .status().expect("cannot format file");
    };

    out.into()
}

/// Executes a statement while allowing it to participate in
/// Statement Deletion (SDL) mutations.
///
/// This macro normally expands to the supplied statement unchanged.
///
/// During mutation testing, the mutation visitor may replace:
///
/// ```rust,ignore
/// sdl!(some_statement();)
/// ```
///
/// with:
///
/// ```rust,ignore
/// sdl!()
/// ```
///
/// allowing the statement to be effectively removed from the mutated
/// program.
///
/// # Example
///
/// ```rust,ignore
/// sdl! {
///     counter += 1;
/// }
/// ```
///
/// expands to:
///
/// ```rust,ignore
/// counter += 1;
/// ```
///
/// # Mutation Support
///
/// Used by the `SDL` (Statement Deletion) mutation operator.
#[proc_macro]
pub fn sdl(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
/// Marks a statement as excluded from mutation testing.
///
/// This macro expands to the supplied statement unchanged and serves
/// as a semantic marker indicating that the enclosed statement should
/// be ignored by mutation operators.
///
/// # Example
///
/// ```rust,ignore
/// ignore! {
///     println!("debug output");
/// }
/// ```
///
/// expands to:
///
/// ```rust,ignore
/// println!("debug output");
/// ```
///
/// # Intended Usage
///
/// This macro can be used to wrap:
///
/// - Logging statements
/// - Debug-only code
/// - Non-essential side effects
/// - Statements that should not affect mutation scores
///
/// # Notes
///
/// The macro itself performs no transformation. Any special behavior
/// depends on mutation visitors or tooling that recognizes the
/// `ignore!` marker.
#[proc_macro]
pub fn ignore(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}