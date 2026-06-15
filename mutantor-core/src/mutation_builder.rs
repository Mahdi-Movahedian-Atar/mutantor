use crate::mutation_operators::MutationOperators;
use proc_macro2::Ident;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseBuffer};
use syn::{LitFloat, LitInt, LitStr, Token};

/// Configuration for mutation test generation.
///
/// This structure is produced by parsing the arguments supplied to
/// the `generate_mutants` attribute macro.
///
/// # Example
///
/// ```rust,ignore
/// #[generate_mutants(
///     AOR,
///     ROR,
///     IPVR,
///     path = "src/lib.rs",
///     m_count = 2,
///     c_count = 3,
///     chance = 1,
///     acc = 80
/// )]
/// ```
///
/// # Fields
///
/// - `operators` – Enabled mutation operators.
/// - `path` – Optional path used by the mutation framework.
/// - `mutation_count` – Number of mutation iterations per operator.
/// - `combination_count` – Number of simultaneous mutation combinations.
/// - `mutation_chance` – Probability that a mutation is applied.
/// - `acceptable_score` – Minimum mutation score required for the test
///   to pass.
#[derive(Debug)]
pub struct MutationBuilder {
    /// Enabled mutation operators.
    ///
    /// Duplicate operators are automatically removed because a
    /// [`HashSet`] is used.
    pub operators: HashSet<MutationOperators>,

    /// Optional source file path.
    ///
    /// This may be used by the mutation framework to locate additional
    /// source code or configuration information.
    pub path: Option<PathBuf>,

    /// Number of mutations generated for each enabled operator.
    ///
    /// Default: `1`
    pub mutation_count: usize,

    /// Number of operator combinations generated during mutation
    /// testing.
    ///
    /// Default: `3`
    pub combination_count: usize,

    /// Probability that an eligible mutation will be applied.
    ///
    /// Valid values are typically in the range `0.0..=1.0`.
    ///
    /// Default: `0.75`
    pub mutation_chance: f64,

    /// Minimum mutation score required for generated tests to pass.
    ///
    /// Expressed as a percentage.
    ///
    /// Default: `75.0`
    pub acceptable_score: f64,

    pub use_acoc: bool,
}

/// Parses the configuration supplied to the mutation-testing
/// attribute macro.
///
/// # Supported Arguments
///
/// ## Mutation Operators
///
/// ```text
/// AOD
/// AOI
/// AOR
/// COD
/// COI
/// COR
/// LOD
/// LOI
/// LOR
/// ROR
/// SDL
/// SOR
/// IPVR
/// IPEX
/// IMCD
/// ```
///
/// ## Configuration Options
///
/// ### path
///
/// Specifies a file path to save an instance of the mutation test.
///
/// ```rust,ignore
/// path = "src/lib"
/// ```
///
/// ### m_count
///
/// Number of mutations generated per operator.
///
/// ```rust,ignore
/// m_count = 5
/// ```
///
/// ### c_count
///
/// Number of mutation combinations.
///
/// ```rust,ignore
/// c_count = 3
/// ```
///
/// ### chance
///
/// Mutation probability.
///
/// ```rust,ignore
/// chance = 1
/// ```
///
/// **Note:** the current implementation parses this as an integer and
/// stores it in a `f64`.
///
/// ### acc
///
/// Required mutation score percentage.
///
/// ```rust,ignore
/// acc = 80
/// ```
///
/// # Defaults
///
/// If an option is omitted:
///
/// ```text
/// mutation_count   = 1
/// combination_count = 3
/// mutation_chance   = 0.75
/// acceptable_score  = 75.0
/// path              = None
/// ```
///
/// # Errors
///
/// Returns a `syn::Error` if:
///
/// - An unknown mutation operator is encountered.
/// - A configuration value cannot be parsed.
/// - The attribute syntax is invalid.
///
/// Example:
///
/// ```rust,ignore
/// #[generate_mutants(UNKNOWN)]
/// ```
///
/// produces:
///
/// ```text
/// unknown mutation operator
/// ```
impl Parse for MutationBuilder {
    fn parse(input: &ParseBuffer<'_>) -> syn::Result<Self> {
        let mut operators = HashSet::new();
        let mut path = None;

        let mut mutation_count = 1;
        let mut combination_count = 3;
        let mut mutation_chance = 0.75;
        let mut acceptable_score = 75.0;

        let mut use_acoc = false;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let op = match ident.to_string().as_str() {
                "path" => {
                    input.parse::<Token![=]>()?;

                    let value: LitStr = input.parse()?;

                    path = Some(Path::new(&value.value()).to_path_buf());

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
                "use_acoc"=>{
                    use_acoc = true;
                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
                "m_count" => {
                    input.parse::<Token![=]>()?;

                    let value: LitInt = input.parse()?;

                    mutation_count = value.base10_parse()?;

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
                "c_count" => {
                    input.parse::<Token![=]>()?;

                    let value: LitInt = input.parse()?;

                    combination_count = value.base10_parse()?;

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
                "acc" => {
                    input.parse::<Token![=]>()?;

                    let value: LitFloat = input.parse()?;

                    acceptable_score = value.base10_parse()?;

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
                "chance" => {
                    input.parse::<Token![=]>()?;

                    let value: LitFloat = input.parse()?;

                    mutation_chance = value.base10_parse()?;

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
                "AOR" => MutationOperators::AOR,
                "AOI" => MutationOperators::AOI,
                "AOD" => MutationOperators::AOD,
                "COD" => MutationOperators::COD,
                "COI" => MutationOperators::COI,
                "COR" => MutationOperators::COR,
                "LOD" => MutationOperators::LOD,
                "LOI" => MutationOperators::LOI,
                "LOR" => MutationOperators::LOR,
                "ROR" => MutationOperators::ROR,
                "SDL" => MutationOperators::SDL,
                "SOR" => MutationOperators::SOR,
                "IPVR" => MutationOperators::IPVR,
                "IPEX" => MutationOperators::IPEX,
                "IMCD" => MutationOperators::IMCD,

                _ => {
                    return Err(syn::Error::new(ident.span(), "unknown mutation operator"));
                }
            };

            operators.insert(op);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            use_acoc,
            operators,
            path,
            mutation_count,
            combination_count,
            acceptable_score,
            mutation_chance,
        })
    }
}
