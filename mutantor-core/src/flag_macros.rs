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
#[macro_export]
macro_rules! sdl {
    {$s: stmt} => {
        $s
    };
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
#[macro_export]
macro_rules! ignore {
    {$s: stmt} => {
        $s
    };
}