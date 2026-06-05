use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents the supported mutation operators used by the mutation
/// testing framework.
///
/// Each variant corresponds to a specific mutation strategy that can
/// be applied to source code during mutant generation.
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
///
/// let op = MutationOperators::from_str("AOR").unwrap();
/// assert_eq!(op, MutationOperators::AOR);
/// ```
///
/// ```rust
/// let op = MutationOperators::ROR;
/// assert_eq!(op.to_string(), "ROR");
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MutationOperators {
    /// Arithmetic Operator Deletion.
    ///
    /// Removes an arithmetic operator from an expression.
    AOD,

    /// Arithmetic Operator Insertion.
    ///
    /// Inserts a unary arithmetic operator where applicable.
    AOI,

    /// Arithmetic Operator Replacement.
    ///
    /// Replaces one arithmetic operator with another.
    AOR,

    /// Conditional Operator Deletion.
    ///
    /// Removes a conditional operator from an expression.
    COD,

    /// Conditional Operator Insertion.
    ///
    /// Inserts a conditional operator into an expression.
    COI,

    /// Conditional Operator Replacement.
    ///
    /// Replaces one conditional operator with another.
    COR,

    /// Logical Operator Deletion.
    ///
    /// Removes a logical operator from an expression.
    LOD,

    /// Logical Operator Insertion.
    ///
    /// Inserts a logical operator into an expression.
    LOI,

    /// Logical Operator Replacement.
    ///
    /// Replaces one logical operator with another.
    LOR,

    /// Relational Operator Replacement.
    ///
    /// Replaces relational operators such as `==`, `!=`, `<`, `>`,
    /// `<=`, and `>=`.
    ROR,

    /// Statement Deletion.
    ///
    /// Removes an executable statement from the program.
    SDL,

    /// Shift Operator Replacement.
    ///
    /// Replaces bitwise shift operators with alternative shift operators.
    SOR,

    /// Instance Property Value Replacement.
    ///
    /// Replaces field or property values with generated alternatives.
    IPVR,

    /// Instance Property Expression Replacement.
    ///
    /// Replaces property access expressions with alternative expressions.
    IPEX,

    /// Instance Method Call Deletion.
    ///
    /// Removes method invocations where doing so is syntactically valid.
    IMCD,
}

/// Converts a [`MutationOperators`] value into its canonical string
/// representation.
///
/// The output matches the enum variant name exactly.
///
/// # Examples
///
/// ```rust
/// let op = MutationOperators::AOR;
/// assert_eq!(op.to_string(), "AOR");
/// ```
impl Display for MutationOperators {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            MutationOperators::AOD => "AOD",
            MutationOperators::AOI => "AOI",
            MutationOperators::AOR => "AOR",
            MutationOperators::COD => "COD",
            MutationOperators::COI => "COI",
            MutationOperators::COR => "COR",
            MutationOperators::LOD => "LOD",
            MutationOperators::LOI => "LOI",
            MutationOperators::LOR => "LOR",
            MutationOperators::ROR => "ROR",
            MutationOperators::SDL => "SDL",
            MutationOperators::SOR => "SOR",
            MutationOperators::IPVR => "IPVR",
            MutationOperators::IPEX => "IPEX",
            MutationOperators::IMCD => "IMCD",
        };

        write!(f, "{name}")
    }
}

/// Parses a mutation operator from its string representation.
///
/// Parsing is case-insensitive and ignores leading/trailing whitespace.
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
///
/// assert_eq!(
///     MutationOperators::from_str("aor").unwrap(),
///     MutationOperators::AOR
/// );
///
/// assert_eq!(
///     MutationOperators::from_str("  ROR  ").unwrap(),
///     MutationOperators::ROR
/// );
/// ```
///
/// # Errors
///
/// Returns an error if the supplied string does not correspond to a
/// known mutation operator.
///
/// ```rust
/// use std::str::FromStr;
///
/// assert!(MutationOperators::from_str("INVALID").is_err());
/// ```
impl FromStr for MutationOperators {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "AOD" => Ok(Self::AOD),
            "AOI" => Ok(Self::AOI),
            "AOR" => Ok(Self::AOR),
            "COD" => Ok(Self::COD),
            "COI" => Ok(Self::COI),
            "COR" => Ok(Self::COR),
            "LOD" => Ok(Self::LOD),
            "LOI" => Ok(Self::LOI),
            "LOR" => Ok(Self::LOR),
            "ROR" => Ok(Self::ROR),
            "SDL" => Ok(Self::SDL),
            "SOR" => Ok(Self::SOR),
            "IPVR" => Ok(Self::IPVR),
            "IPEX" => Ok(Self::IPEX),
            "IMCD" => Ok(Self::IMCD),
            _ => Err(format!("Unknown mutation operator: {s}")),
        }
    }
}