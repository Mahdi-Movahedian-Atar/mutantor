use crate::mutation::Mutation;
use crate::mutation_builder::MutationBuilder;
use crate::mutation_input::{InputType, mutation_input};
use crate::mutation_operators::MutationOperators;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use rand::RngExt;
use rand::prelude::SliceRandom;
use syn::visit_mut::VisitMut;
use syn::{FnArg, ItemFn, Pat, PatType, Type};
/// Collects generated mutant functions.
///
/// This structure is used during procedural macro expansion to store
/// generated mutant implementations before they are emitted into the
/// final token stream.
///
/// Each entry in `mutants` contains the source code for a mutated
/// function represented as a [`TokenStream`].
#[derive(Debug)]
pub struct MutationCollector {
    /// Generated mutant function definitions.
    pub mutants: Vec<TokenStream>,
}

/// Generates a mutation-testing module for a function.
///
/// This function is the core of the mutation testing framework. It:
///
/// 1. Generates randomized input values for the target function.
/// 2. Creates mutated versions of the function using the configured
///    mutation operators.
/// 3. Executes the original and mutated functions.
/// 4. Compares their outputs.
/// 5. Calculates a mutation score.
/// 6. Generates a test that fails when the mutation score falls below
///    the configured threshold.
///
/// The generated code is emitted as a Rust test module containing:
///
/// - A mutation test function.
/// - All generated mutant implementations.
///
/// # Parameters
///
/// * `mutation_data` - Mutation configuration describing:
///   - Enabled operators.
///   - Mutation count.
///   - Mutation combinations.
///   - Mutation probability.
///   - Required mutation score.
///
/// * `func` - Function that will be mutated.
///
/// # Generated Workflow
///
/// ## Input Generation
///
/// For each function parameter:
///
/// ```rust,ignore
/// let mut value = Type::new_mutable(&mut rng);
/// ```
///
/// is generated.
///
/// Generated values are reused for both the original function and its
/// mutants.
///
/// ## Mutant Generation
///
/// A mutated clone of the original function is created for every
/// mutation configuration.
///
/// Example:
///
/// ```rust,ignore
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// ```
///
/// may generate:
///
/// ```rust,ignore
/// fn add_0(a: i32, b: i32) -> i32 {
///     a - b
/// }
/// ```
///
/// when using the `AOR` mutation operator.
///
/// ## Execution
///
/// The generated test executes:
///
/// ```rust,ignore
/// original_result == mutant_result
/// ```
///
/// If the results are identical:
///
/// ```text
/// mutation survived
/// ```
///
/// Otherwise:
///
/// ```text
/// mutation killed
/// ```
///
/// and the mutation score is increased.
///
/// ## Mutation Score
///
/// The score is calculated as:
///
/// ```text
/// killed_mutants / total_mutants * 100
/// ```
///
/// The generated test asserts:
///
/// ```rust,ignore
/// score >= acceptable_score
/// ```
///
/// # Supported Mutation Categories
///
/// ## Arithmetic Operators
///
/// - AOD
/// - AOI
/// - AOR
///
/// ## Conditional Operators
///
/// - COD
/// - COI
/// - COR
///
/// ## Logical Operators
///
/// - LOD
/// - LOI
/// - LOR
///
/// ## Relational Operators
///
/// - ROR
///
/// ## Statement Mutations
///
/// - SDL
///
/// ## Shift Operator Mutations
///
/// - SOR
///
/// ## Input Parameter Mutations
///
/// - IPVR
/// - IPEX
///
/// ## Invocation Mutations
///
/// - IMCD
///
/// # Generated Output
///
/// Produces code similar to:
///
/// ```rust,ignore
/// #[cfg(test)]
/// mod my_function_test {
///     #[test]
///     fn test() {
///         // mutation execution
///     }
///
///     fn my_function_0(...) { ... }
///     fn my_function_1(...) { ... }
/// }
/// ```
///
/// # Notes
///
/// - Mutation combinations are randomized.
/// - Generated inputs depend on the [`Mutable`] trait implementations.
/// - Mutants are created by cloning and transforming the original
///   function AST using `syn::VisitMut`.
/// - Function parameters passed by reference are automatically handled
///   through [`InputType`].
///
/// # Panics
///
/// This function may panic if:
///
/// - A mutation operator is selected that is not handled by the
///   mutation visitor.
/// - Input mutation generation selects an invalid replacement.
/// - Internal mutation configuration becomes inconsistent.
///
/// # Returns
///
/// Returns a [`TokenStream`] containing the generated mutation-testing
/// module and all mutant function implementations.
pub fn mutation_collector(mutation_data: &MutationBuilder, func: &ItemFn) -> TokenStream {
    let mut final_code = TokenStream::new();
    let mut mutation_call = TokenStream::new();
    let mut mutation_functions = TokenStream::new();

    let mut rng = rand::rng();
    let mut operators = Vec::new();

    let mut variables = Vec::new();

    for _ in 0..mutation_data.combination_count  {
        operators.push(Vec::new());
        for _ in 0..mutation_data.mutation_count  {
            operators
                .last_mut()
                .unwrap()
                .extend(mutation_data.operators.clone());
        }
        operators.last_mut().unwrap().shuffle(&mut rng);
    }

    for arg in &func.sig.inputs {
        let FnArg::Typed(PatType { pat, ty, .. }) = arg else {
            continue;
        };

        let Pat::Ident(pat_ident) = &**pat else {
            continue;
        };

        let name = &pat_ident.ident;

        let (name, rf, ty) = match &**ty {
            Type::Reference(r) => (
                name,
                if r.mutability.is_some() {
                    InputType::MutRef
                } else {
                    InputType::Ref
                },
                r.elem.to_token_stream(),
            ),
            other => (name, InputType::Own, other.to_token_stream()),
        } ;

        final_code = quote! {
            #final_code
            let mut #name = #ty::new_mutable(&mut rng);
        };
        variables.push((name, rf, ty))
    }

    let main_function_call =
        mutation_input(&variables, &0.0, &Vec::new(), &func.sig.ident, &mut rng);

    for i in 0..(mutation_data.mutation_count * mutation_data.operators.len())  {
        let new_function_name = format!("{}_{}", func.sig.ident, i);
        let new_function_ident = syn::Ident::new(&new_function_name, func.sig.ident.span());
        let mut mutation_names = String::new();

        let mut ao_operators: Vec<MutationOperators> = Vec::new();
        let mut co_operators: Vec<MutationOperators> = Vec::new();
        let mut lo_operators: Vec<MutationOperators> = Vec::new();
        let mut ip_operators: Vec<MutationOperators> = Vec::new();
        let mut ro_operators: bool = false;
        let mut sd_operators: bool = false;
        let mut so_operators: bool = false;
        let mut imcd_operators: bool = false;
        let mut imcd_chance = (mutation_data.combination_count + 1) as f64;
        for j in 0..mutation_data.combination_count  {
            match operators[j][i] {
                MutationOperators::AOD => ao_operators.push(MutationOperators::AOD),
                MutationOperators::AOI => ao_operators.push(MutationOperators::AOI),
                MutationOperators::AOR => ao_operators.push(MutationOperators::AOR),
                MutationOperators::COD => co_operators.push(MutationOperators::COD),
                MutationOperators::COI => co_operators.push(MutationOperators::COI),
                MutationOperators::COR => co_operators.push(MutationOperators::COR),
                MutationOperators::LOD => lo_operators.push(MutationOperators::LOD),
                MutationOperators::LOI => lo_operators.push(MutationOperators::LOI),
                MutationOperators::LOR => lo_operators.push(MutationOperators::LOR),
                MutationOperators::ROR => ro_operators = true,
                MutationOperators::SDL => sd_operators = true,
                MutationOperators::SOR => so_operators = true,
                MutationOperators::IPVR => ip_operators.push(MutationOperators::IPVR),
                MutationOperators::IPEX => ip_operators.push(MutationOperators::IPEX),
                MutationOperators::IMCD => {
                    imcd_operators = true;
                    imcd_chance -= 1f64;
                }
            };
            mutation_names = format!("{} {},",mutation_names,operators[j][i]);
        }
        if imcd_operators && rng.random_bool(mutation_data.mutation_chance / imcd_chance) {
            mutation_call = quote! {
                #mutation_call
                if(#main_function_call == Mutable::new_mutable(&mut rng)){
                    println!("mutation {} survived _ {}",#i, #mutation_names);
                }else {
                    println!("mutation {} killed _ {}",#i, #mutation_names);
                    score += 1f64;
                }
            };
            continue;
        }
        let new_function_call = mutation_input(
            &variables,
            &mutation_data.mutation_chance,
            &ip_operators,
            &new_function_ident,
            &mut rng,
        );
        mutation_call = quote! {
            #mutation_call
            if(#main_function_call == #new_function_call){
                println!("mutation {} survived _ {}",#i, #mutation_names);
            }else {
                println!("mutation {} killed _ {}",#i, #mutation_names);
                score += 1f64;
            }
        };

        let mut new_func = func.clone();
        new_func.sig.ident = new_function_ident;

        Mutation {
            ao_operators,
            co_operators,
            lo_operators,
            ro_operators,
            sd_operators,
            so_operators,
            mutation_chance: &mutation_data.mutation_chance,
            rng: &mut rng,
        }
        .visit_block_mut(&mut new_func.block);
        mutation_functions = quote! {
            #mutation_functions
            #new_func
        }
    }

    let mod_name = syn::Ident::new(
        &format!("{}_test", func.sig.ident,),
        proc_macro2::Span::call_site(),
    );
    let mutations = mutation_data.mutation_count as f64 * mutation_data.operators.len() as f64;
    let acceptable = mutation_data.acceptable_score;
    quote! {
        #[cfg(test)]
        #[allow(unused)]
        mod #mod_name{
            use super::*;
            use mutantor::rand;
            use mutantor::Mutable;
            #[test]
            fn test(){
                let mut  score = 0f64;
                let mut rng = rand::rng();
                #final_code
                #mutation_call
                println!("score {}", (score/#mutations) * 100f64);
                assert!((score/#mutations) * 100f64 >= #acceptable)
            }
            #mutation_functions
        }
    }
}
