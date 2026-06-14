use crate::mutation_operators::MutationOperators;
use rand::RngExt;
use rand::prelude::ThreadRng;
use syn::visit_mut::VisitMut;
use syn::{BinOp, ExprBinary, Macro, parse_quote};
/// Identifies the operator form of a binary expression.
///
/// Some mutation operators behave differently depending on whether the
/// operator is a regular binary operator (`+`, `&`, `<<`) or an
/// assignment operator (`+=`, `&=`, `<<=`).
enum AT {
    /// Assignment-style operator.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a += b;
    /// a &= b;
    /// a <<= b;
    /// ```
    EQ,

    /// Regular binary operator.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a + b
    /// a & b
    /// a << b
    /// ```
    RL,
}

/// Categorizes binary expressions for mutation processing.
///
/// During AST traversal, operators are grouped into mutation
/// categories so the appropriate mutation strategy can be applied.
enum ExType {
    /// Arithmetic operation.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a + b
    /// a -= b
    /// ```
    AO(AT),

    /// Conditional operation.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a && b
    /// a || b
    /// ```
    CO,

    /// Logical/bitwise operation.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a & b
    /// a | b
    /// a ^= b
    /// ```
    LO(AT),

    /// Relational comparison.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a < b
    /// a >= b
    /// ```
    RO,

    /// Shift operation.
    ///
    /// Examples:
    ///
    /// ```rust
    /// a << b
    /// a >>= b
    /// ```
    SO(AT),

    /// Unsupported or non-mutable expression type.
    ///
    /// Expressions mapped to this variant are ignored by the mutation
    /// engine.
    None,
}

/// AST mutation visitor.
///
/// Traverses a Rust syntax tree and applies mutation operators to
/// matching expressions.
///
/// This visitor is used to generate mutant versions of a function for
/// mutation testing. Each enabled operator category may modify nodes
/// encountered during traversal according to the configured mutation
/// probability.
///
/// # Lifetime
///
/// The lifetime parameter ties the visitor to externally-owned
/// configuration and random number generator instances.
#[derive(Debug)]
pub(crate) struct Mutation<'a> {
    /// Enabled arithmetic mutation operators.
    ///
    /// Supported operators:
    ///
    /// - `AOD`
    /// - `AOI`
    /// - `AOR`
    pub(crate) ao_operators: Vec<MutationOperators>,

    /// Enabled conditional mutation operators.
    ///
    /// Supported operators:
    ///
    /// - `COD`
    /// - `COI`
    /// - `COR`
    pub(crate) co_operators: Vec<MutationOperators>,

    /// Enabled logical mutation operators.
    ///
    /// Supported operators:
    ///
    /// - `LOD`
    /// - `LOI`
    /// - `LOR`
    pub(crate) lo_operators: Vec<MutationOperators>,

    /// Enables relational operator replacement (`ROR`).
    ///
    /// When enabled, comparison operators such as `<`, `>`, `<=`, and
    /// `>=` may be replaced with alternative comparisons.
    pub(crate) ro_operators: bool,

    /// Enables statement deletion (`SDL`).
    ///
    /// This affects recognized `sdl!` macro invocations.
    pub(crate) sd_operators: bool,

    /// Enables shift operator replacement (`SOR`).
    ///
    /// Shift operators such as `<<`, `>>`, `<<=`, and `>>=` may be
    /// replaced with alternative shift operations.
    pub(crate) so_operators: bool,

    /// Probability that a matching expression will be mutated.
    ///
    /// Expected range:
    ///
    /// ```text
    /// 0.0 <= mutation_chance <= 1.0
    /// ```
    ///
    /// Examples:
    ///
    /// - `0.0` = never mutate
    /// - `0.5` = mutate approximately half of eligible expressions
    /// - `1.0` = always mutate
    pub(crate) mutation_chance: &'a f64,

    /// Random number generator used for mutation selection.
    ///
    /// This generator controls:
    ///
    /// - Whether a mutation occurs.
    /// - Which mutation operator is chosen.
    /// - Which replacement variant is applied.
    pub(crate) rng: &'a mut ThreadRng,
}
/// Applies mutation operators to a Rust syntax tree.
///
/// This visitor traverses binary expressions and selected macro
/// invocations, replacing operators or expressions according to the
/// configured mutation operators.
///
/// Mutations are applied probabilistically using
/// [`Mutation::mutation_chance`].
///
/// # Supported Mutations
///
/// ## Arithmetic
///
/// - `AOD` — Arithmetic Operator Deletion
/// - `AOI` — Arithmetic Operator Insertion
/// - `AOR` — Arithmetic Operator Replacement
///
/// ## Conditional
///
/// - `COD` — Conditional Operator Deletion
/// - `COI` — Conditional Operator Insertion
/// - `COR` — Conditional Operator Replacement
///
/// ## Logical
///
/// - `LOD` — Logical Operator Deletion
/// - `LOI` — Logical Operator Insertion
/// - `LOR` — Logical Operator Replacement
///
/// ## Relational
///
/// - `ROR` — Relational Operator Replacement
///
/// ## Shift
///
/// - `SOR` — Shift Operator Replacement
///
/// ## Statement
///
/// - `SDL` — Statement Deletion
impl<'a> VisitMut for Mutation<'a> {
    /// Visits and potentially mutates binary expressions.
    ///
    /// Every supported binary operator is classified into one of the
    /// internal expression categories:
    ///
    /// - Arithmetic (`AO`)
    /// - Conditional (`CO`)
    /// - Logical (`LO`)
    /// - Relational (`RO`)
    /// - Shift (`SO`)
    ///
    /// Unsupported operators are ignored.
    ///
    /// # Mutation Flow
    ///
    /// 1. Randomly decide whether the expression should be mutated.
    /// 2. Determine the expression category.
    /// 3. Select an enabled mutation operator.
    /// 4. Apply the mutation.
    /// 5. Continue recursive traversal.
    ///
    /// # Arithmetic Mutations
    ///
    /// ```rust,ignore
    /// a + b
    /// ```
    ///
    /// may become:
    ///
    /// ```rust,ignore
    /// a - b
    /// a + 0
    /// -a + b
    /// ```
    ///
    /// # Conditional Mutations
    ///
    /// ```rust,ignore
    /// a && b
    /// ```
    ///
    /// may become:
    ///
    /// ```rust,ignore
    /// a || b
    /// !a || !b
    /// true || true
    /// ```
    ///
    /// # Logical Mutations
    ///
    /// ```rust,ignore
    /// a & b
    /// ```
    ///
    /// may become:
    ///
    /// ```rust,ignore
    /// a | b
    /// a ^ b
    /// !a & b
    /// ```
    ///
    /// # Relational Mutations
    ///
    /// ```rust,ignore
    /// a < b
    /// ```
    ///
    /// may become:
    ///
    /// ```rust,ignore
    /// a > b
    /// a <= b
    /// a >= b
    /// ```
    ///
    /// # Shift Mutations
    ///
    /// ```rust,ignore
    /// a << b
    /// ```
    ///
    /// may become:
    ///
    /// ```rust,ignore
    /// a >> b
    /// ```
    ///
    /// # Notes
    ///
    /// Mutations are selected randomly from the enabled operator
    /// collections.
    fn visit_expr_binary_mut(&mut self, node: &mut ExprBinary) {
        if !self.rng.random_bool(self.mutation_chance.clone()) {
            syn::visit_mut::visit_expr_binary_mut(self, node);
            return;
        }

        let left = &mut node.left;
        let right = &mut node.right;

        let ex_type = match &mut node.op {
            BinOp::Add(_) | BinOp::Sub(_) | BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => {
                ExType::AO(AT::RL)
            }
            BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_) => ExType::AO(AT::EQ),
            BinOp::And(_) | BinOp::Or(_) => ExType::CO,
            BinOp::BitAnd(_) | BinOp::BitOr(_) | BinOp::BitXor(_) => ExType::LO(AT::RL),
            BinOp::BitAndAssign(_) | BinOp::BitOrAssign(_) | BinOp::BitXorAssign(_) => {
                ExType::LO(AT::EQ)
            }
            BinOp::Gt(_) | BinOp::Ge(_) | BinOp::Lt(_) | BinOp::Le(_) => ExType::RO,
            BinOp::Shl(_) | BinOp::Shr(_) => ExType::SO(AT::RL),
            BinOp::ShlAssign(_) | BinOp::ShrAssign(_) => ExType::SO(AT::EQ),
            _ => ExType::None,
        };
        match ex_type {
            ExType::AO(t) => {
                if self.ao_operators.len() != 0 {
                    match self.ao_operators[self.rng.random_range(0..self.ao_operators.len() )] {
                        MutationOperators::AOD => match t {
                            AT::EQ => {
                                *node = parse_quote!(#left += 0);
                            }
                            AT::RL => {
                                if self.rng.random_bool(0.5) {
                                    *node = parse_quote!(#left + 0);
                                } else {
                                    *node = parse_quote!(0 + #right);
                                }
                            }
                        },
                        MutationOperators::AOI => match t {
                            AT::EQ => {
                                *right = parse_quote!(-#right);
                            }
                            AT::RL => {
                                if self.rng.random_bool(0.5) {
                                    *right = parse_quote!(-#right);
                                } else {
                                    *left = parse_quote!(-#left);
                                }
                            }
                        },
                        MutationOperators::AOR => match t {
                            AT::EQ => {
                                let operators = [
                                    parse_quote!(+=),
                                    parse_quote!(-=),
                                    parse_quote!(/=),
                                    parse_quote!(*=),
                                    parse_quote!(%=),
                                ];
                                node.op = operators[self.rng.random_range(0..operators.len())];
                            }
                            AT::RL => {
                                let operators = [
                                    parse_quote!(+),
                                    parse_quote!(-),
                                    parse_quote!(/),
                                    parse_quote!(*),
                                    parse_quote!(%),
                                ];
                                node.op = operators[self.rng.random_range(0..operators.len())];
                            }
                        },
                        _ => {}
                    }
                }
            }
            ExType::CO => {
                if self.co_operators.len() != 0 {
                    match self.co_operators[self.rng.random_range(0..self.co_operators.len() )] {
                        MutationOperators::COD => {
                            if self.rng.random_bool(0.5) {
                                *right = parse_quote!(#left);
                            } else {
                                *left = parse_quote!(#right);
                            }
                        }
                        MutationOperators::COI => {
                            if self.rng.random_bool(0.5) {
                                *left = parse_quote!(!#left );
                                *right = parse_quote!(!#right );
                                match node.op {
                                    BinOp::Or(_) => node.op = parse_quote!(&&),
                                    _ => node.op = parse_quote!(||),
                                }
                            } else {
                                if self.rng.random_bool(0.5) {
                                    *node = parse_quote!(#node || true);
                                } else {
                                    *node = parse_quote!(#node && false);
                                }
                            }
                        }
                        MutationOperators::COR => {
                            if self.rng.random_bool(0.5) {
                                if self.rng.random_bool(0.5) {
                                    node.op = parse_quote!(||);
                                } else {
                                    node.op = parse_quote!(&&);
                                }
                            } else {
                                if self.rng.random_bool(0.5) {
                                    *node = parse_quote!(true || true);
                                } else {
                                    *node = parse_quote!(false || false);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            ExType::LO(t) => {
                if self.lo_operators.len() != 0 {
                    match self.lo_operators[self.rng.random_range(0..self.lo_operators.len() )] {
                        MutationOperators::LOD => match t {
                            AT::EQ => {
                                *node = parse_quote!(#left += 0);
                            }
                            AT::RL => {
                                if self.rng.random_bool(0.5) {
                                    *node = parse_quote!(0 + #right);
                                } else {
                                    *node = parse_quote!(#left + 0);
                                }
                            }
                        },
                        MutationOperators::LOI => match t {
                            AT::EQ => {
                                *right = parse_quote!(!#right);
                            }
                            AT::RL => {
                                if self.rng.random_bool(0.5) {
                                    *right = parse_quote!(!#right);
                                } else {
                                    *left = parse_quote!(!#left);
                                }
                            }
                        },
                        MutationOperators::LOR => match t {
                            AT::EQ => {
                                let operators =
                                    [parse_quote!(&=), parse_quote!(|=), parse_quote!(^=)];
                                node.op = operators[self.rng.random_range(0..operators.len())];
                            }
                            AT::RL => {
                                let operators = [parse_quote!(&), parse_quote!(|), parse_quote!(^)];
                                node.op = operators[self.rng.random_range(0..operators.len())];
                            }
                        },
                        _ => {}
                    }
                }
            }
            ExType::RO => {
                if self.ro_operators {
                    match self.rng.random_range(0..5) {
                        0 => {
                            node.op = parse_quote!(<);
                        }
                        1 => {
                            node.op = parse_quote!(>);
                        }
                        2 => {
                            node.op = parse_quote!(<=);
                        }
                        3 => {
                            node.op = parse_quote!(>=);
                        }
                        _ => {
                            *node = parse_quote!(true || true);
                        }
                    }
                }
            }
            ExType::SO(t) => {
                if self.so_operators {
                    match t {
                        AT::EQ => {
                            if self.rng.random_bool(0.5) {
                                node.op = parse_quote!(<<=);
                            } else {
                                node.op = parse_quote!(>>=);
                            }
                        }
                        AT::RL => {
                            if self.rng.random_bool(0.5) {
                                node.op = parse_quote!(<<);
                            } else {
                                node.op = parse_quote!(>>);
                            }
                        }
                    }
                }
            }
            ExType::None => {}
        }

        syn::visit_mut::visit_expr_binary_mut(self, node);
    }
    /// Visits macro invocations and applies statement deletion
    /// mutations.
    ///
    /// If SDL mutations are enabled and the mutation probability check
    /// succeeds, occurrences of:
    ///
    /// ```rust,ignore
    /// sdl!(...)
    /// ```
    ///
    /// are replaced with:
    ///
    /// ```rust,ignore
    /// sdl!()
    /// ```
    ///
    /// This allows statement removal behavior to be implemented by the
    /// user-defined `sdl!` macro.
    ///
    /// # Notes
    ///
    /// Only macros whose path is exactly `sdl` are modified.
    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        if self.sd_operators && self.rng.random_bool(self.mutation_chance.clone()) {
            if mac.path.is_ident("sdl") {
                *mac = parse_quote!(sdl!());
                return;
            }
        }
    }
}
