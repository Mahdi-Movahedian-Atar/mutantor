use crate::mutation_operators::MutationOperators;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use rand::RngExt;
use rand::rngs::ThreadRng;

/// Describes how a function parameter is passed to the target function.
pub(crate) enum InputType {
    /// The parameter is passed by value.
    ///
    /// Example:
    ///
    /// ```rust
    /// fn foo(x: i32) {}
    /// ```
    Own,

    /// The parameter is passed as an immutable reference.
    ///
    /// Example:
    ///
    /// ```rust
    /// fn foo(x: &i32) {}
    /// ```
    Ref,

    /// The parameter is passed as a mutable reference.
    ///
    /// Example:
    ///
    /// ```rust
    /// fn foo(x: &mut i32) {}
    /// ```
    MutRef,
}

/// Generates a function call with optionally mutated input arguments.
///
/// This function is used during mutant generation to construct a call
/// expression for the target function. Each input may either:
///
/// - Be passed unchanged.
/// - Be replaced with a newly generated value (`IPVR`).
/// - Be replaced with another variable of the same type (`IPEX`).
///
/// The mutation applied to a parameter is chosen randomly according to
/// `mutation_chance`.
///
/// # Parameters
///
/// * `variables` - Available function inputs. Each tuple contains:
///   - The variable identifier.
///   - The parameter passing mode ([`InputType`]).
///   - The parameter type.
///
/// * `mutation_chance` - Probability that a given argument will be mutated.
///
/// * `ip_operators` - Enabled input-parameter mutation operators.
///
/// * `function_name` - Name of the function being invoked.
///
/// * `rng` - Random number generator used for mutation decisions.
///
/// # Supported Mutation Operators
///
/// ## IPVR (Input Parameter Value Replacement)
///
/// Replaces an argument with a newly generated value:
///
/// ```rust,ignore
/// foo(x)
/// ```
///
/// becomes:
///
/// ```rust,ignore
/// foo(i32::new_mutable(&mut rng))
/// ```
///
/// ## IPEX (Input Parameter Expression Replacement)
///
/// Replaces an argument with another variable of the same type:
///
/// ```rust,ignore
/// foo(x)
/// ```
///
/// becomes:
///
/// ```rust,ignore
/// foo(y)
/// ```
///
/// where `x` and `y` have identical types.
///
/// # Return Value
///
/// Returns a [`TokenStream`] representing the generated function call.
///
/// Example output:
///
/// ```rust,ignore
/// my_function(
///     arg1.clone_mutable(),
///     &arg2,
///     &mut arg3.clone_mutable(),
/// )
/// ```
///
/// # Panics
///
/// Panics if an unsupported mutation operator is encountered in
/// `ip_operators`.
///
/// Panics if `IPEX` is selected and no compatible replacement variable
/// exists.
pub fn mutation_input(
    variables: &Vec<(&Ident, InputType, TokenStream)>,
    mutation_chance: &f64,
    ip_operators: &Vec<MutationOperators>,
    function_name: &Ident,
    rng: &mut ThreadRng,
) -> TokenStream {
    let mut final_in = TokenStream::new();

    for i in variables {
        let inp = if ip_operators.len() != 0 && rng.random_bool(mutation_chance.clone()) {
            match ip_operators[rng.random_range(..ip_operators.len())] {
                MutationOperators::IPVR => {
                    let ty = &i.2;
                    quote! {#ty::new_mutable(&mut rng)}
                }
                MutationOperators::IPEX => {
                    let posable_inputs = variables
                        .iter()
                        .filter_map(|x| {
                            if x.2.to_string() == i.2.to_string() {
                                return Some(x.0);
                            }
                            None
                        })
                        .collect::<Vec<_>>();

                    let inp = posable_inputs[rng.random_range(..posable_inputs.len())];
                    quote! {#inp}
                }
                _ => panic!(),
            }
        } else {
            let inp = i.0;
            quote! {#inp}
        };

        match i.1 {
            InputType::Own => {
                final_in = quote! {#final_in #inp.clone_mutable(),}
            }
            InputType::Ref => {
                final_in = quote! {#final_in & #inp,}
            }
            InputType::MutRef => {
                final_in = quote! {#final_in &mut #inp.clone_mutable(),}
            }
        }
    }

    quote!(#function_name(#final_in))
}