use rand::RngExt;
use rand::prelude::ThreadRng;
use std::collections::{HashMap, HashSet};

/// Trait for types that can generate randomized values for mutation testing.
///
/// `Mutable` is primarily intended for fuzzing, property testing,
/// and mutation testing scenarios where arbitrary values need to
/// be generated automatically.
///
/// # Example
///
/// ```rust
/// use rand::rng;
///
/// let mut rng = rng();
/// let value = i32::new_mutable(&mut rng);
/// ```
pub trait Mutable: Clone {
    /// Creates a new randomized instance of the implementing type.
    ///
    /// The generated value should be suitable for testing and mutation
    /// purposes, but does not necessarily cover the entire valid range
    /// of the type.
    fn new_mutable(rng: &mut ThreadRng) -> Self;

    /// Creates a mutable clone of the current value.
    ///
    /// By default this is equivalent to calling [`Clone::clone`].
    fn clone_mutable(&self) -> Self {
        Clone::clone(&self)
    }
    /// Generates a collection-based mutation.
    ///
    /// This method is intended for mutation operators that require
    /// alternative values derived from an existing collection of inputs.
    ///
    /// The default implementation ignores the provided input values and
    /// generates a single random instance using [`Mutable::new_mutable`].
    ///
    /// # Parameters
    ///
    /// * `input` - Existing values that may be used to generate
    ///   alternative candidates.
    /// * `rng` - Random number generator.
    ///
    /// # Returns
    ///
    /// A vector containing candidate mutation values.
    ///
    /// # Default Implementation
    ///
    /// ```rust,ignore
    /// vec![Self::new_mutable(rng)]
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// impl Mutable for MyType {
    ///     fn acoc(
    ///         input: &[Self],
    ///         rng: &mut ThreadRng,
    ///     ) -> Vec<Self> {
    ///         input.iter().cloned().collect()
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// Custom implementations may use the supplied `input` slice to
    /// generate context-aware mutations instead of purely random values.
    fn acoc(input:&[Self],rng: &mut ThreadRng)->Vec<Self>{
        vec![Self::new_mutable(rng)]
    }
}

macro_rules! impl_mutable_primitive {
    ($($t:ty),* $(,)?) => {
        $(
            /// Generates a random value in the range `1..10`.
            impl Mutable for $t {
                fn new_mutable(
                    rng: &mut ThreadRng
                ) -> Self {
                    rng.random_range(1 as $t..10 as $t)
                }
                fn acoc(input:&[Self],rng: &mut ThreadRng)->Vec<Self>{
                    if input.len() == 0{
                        return vec![Self::new_mutable(rng)];
                    }
                    let mut out = Vec::from(input);
                    for i in input {
                        let a = i - 1 as $t;
                        let b = i + 1 as $t;
                        out.push(a);
                        out.push(b);
                    }
                    out
                }
            }
        )*
    };
}

impl_mutable_primitive!(
    u8, u16, u32, u64,
    i8, i16, i32, i64,
    f32, f64
);

/// Generates a random `usize` in the range `1..10`.
impl Mutable for usize {
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        rng.random_range(1..10) as usize
    }
    fn acoc(input:&[Self],rng: &mut ThreadRng)->Vec<Self>{
        if input.len() == 0{
            return vec![Self::new_mutable(rng)];
        }
        let mut out = Vec::from(input);
        for i in input {
            let a = i - 1;
            let b = i + 1;
            out.push(a);
            out.push(b);
        }
        out
    }
}

/// Generates a random `isize` in the range `-10..10`.
impl Mutable for isize {
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        rng.random_range(-10..10) as isize
    }
    fn acoc(input:&[Self],rng: &mut ThreadRng)->Vec<Self>{
        if input.len() == 0{
            return vec![Self::new_mutable(rng)];
        }
        let mut out = Vec::from(input);
        for i in input {
            let a = i - 1;
            let b = i + 1;
            out.push(a);
            out.push(b);
        }
        out
    }
}

/// Generates a random boolean value.
impl Mutable for bool {
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        rng.random()
    }
    fn acoc(input:&[Self],rng: &mut ThreadRng)->Vec<Self>{
        if input.len() == 0{
            return vec![Self::new_mutable(rng)];
        }
        let mut out = Vec::from(input);
        for i in input {
            out.push(!i);
        }
        out
    }
}

/// Generates a tuple by independently generating each element.
impl<A, B> Mutable for (A, B)
where
    A: Mutable,
    B: Mutable,
{
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        (A::new_mutable(rng), B::new_mutable(rng))
    }
}

/// Generates a boxed value containing a randomized inner value.
impl<T> Mutable for Box<T>
where
    T: Mutable,
{
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        Box::new(T::new_mutable(rng))
    }
}

/// Randomly generates either:
///
/// - `Some(T)` containing a randomized value.
/// - `None`.
impl<T> Mutable for Option<T>
where
    T: Mutable,
{
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        if rng.random() {
            Some(T::new_mutable(rng))
        } else {
            None
        }
    }
}

/// Generates a random [`HashMap`].
///
/// The map length is chosen randomly in the range `0..10`.
/// Keys and values are generated using their respective
/// [`Mutable`] implementations.
///
/// Duplicate keys may reduce the final map size.
impl<K, V> Mutable for HashMap<K, V>
where
    K: Mutable + Eq + std::hash::Hash,
    V: Mutable,
{
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        let len = rng.random_range(0..10);

        let mut map = HashMap::new();

        for _ in 0..len {
            map.insert(K::new_mutable(rng), V::new_mutable(rng));
        }

        map
    }
}

/// Generates a random [`HashSet`].
///
/// The set length is chosen randomly in the range `0..10`.
/// Elements are generated using their [`Mutable`] implementation.
///
/// Duplicate elements may reduce the final set size.
impl<T> Mutable for HashSet<T>
where
    T: Mutable + Eq + std::hash::Hash,
{
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        let len = rng.random_range(0..10);

        let mut set = HashSet::new();

        for _ in 0..len {
            set.insert(T::new_mutable(rng));
        }

        set
    }
}

/// Generates a random vector.
///
/// The vector length is chosen randomly in the range `0..10`,
/// and each element is generated using its [`Mutable`]
/// implementation.
impl<T> Mutable for Vec<T>
where
    T: Mutable,
{
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        let len = rng.random_range(0..10);

        (0..len).map(|_| T::new_mutable(rng)).collect()
    }
}

/// Generates a random lowercase ASCII string.
///
/// String length is chosen randomly in the range `0..20`.
/// Characters are generated from `a-z`.
impl Mutable for String {
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        let len = rng.random_range(0..20);

        (0..len)
            .map(|_| rng.random_range(b'a'..=b'z') as char)
            .collect()
    }
}