# Mutantor

A procedural mutation testing framework for Rust.

Mutantor helps evaluate the quality of your test suite by automatically generating and executing **mutants**—small modifications of your code designed to simulate common programming mistakes.

A strong test suite should detect and fail when mutant behavior differs from the original implementation. Mutantor measures this using a **mutation score**.

## What is Mutation Testing?

Traditional code coverage only tells you whether code was executed.

Mutation testing goes further:

1. Generate modified versions of your code (mutants).
2. Execute your tests against those mutants.
3. Measure how many mutants were detected ("killed").
4. Calculate a mutation score.

Example:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

A mutant may replace `+` with `-`:

```rust
fn add(a: i32, b: i32) -> i32 {
    a - b
}
```

If your tests fail, the mutant is **killed**.

If your tests still pass, the mutant **survived**, indicating missing test coverage.

---

## Features

- Arithmetic operator mutations
- Conditional operator mutations
- Logical operator mutations
- Relational operator mutations
- Shift operator mutations
- Statement deletion mutations
- Input parameter mutations
- Randomized test input generation
- Configurable mutation scoring
- ACOC based input
- AI-assisted mutation planning (optional)

---

## Installation

```toml
[dependencies]
mutantor = "0.2"
```

Enable AI-assisted mutation planning:

```toml
[dependencies]
mutantor = { version = "0.2", features = ["ai"] }
```

---

## Quick Start

```rust
use mutantor::generate_mutants;

#[generate_mutants(
    AOR,
    ROR,
    IPVR,
    acc = 80,
    m_count = 2
)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run tests:

```bash
cargo test
```

Mutantor will generate mutant functions, execute them, and report a mutation score.

---

## Mutation Operators

### Arithmetic Operators

| Operator | Description |
|-----------|-------------|
| AOD | Arithmetic Operator Deletion |
| AOI | Arithmetic Operator Insertion |
| AOR | Arithmetic Operator Replacement |

### Conditional Operators

| Operator | Description |
|-----------|-------------|
| COD | Conditional Operator Deletion |
| COI | Conditional Operator Insertion |
| COR | Conditional Operator Replacement |

### Logical Operators

| Operator | Description |
|-----------|-------------|
| LOD | Logical Operator Deletion |
| LOI | Logical Operator Insertion |
| LOR | Logical Operator Replacement |

### Relational Operators

| Operator | Description |
|-----------|-------------|
| ROR | Relational Operator Replacement |

### Shift Operators

| Operator | Description |
|-----------|-------------|
| SOR | Shift Operator Replacement |

### Statement Mutations

| Operator | Description |
|-----------|-------------|
| SDL | Statement Deletion |

### Input Parameter Mutations

| Operator | Description |
|-----------|-------------|
| IPVR | Input Parameter Value Replacement |
| IPEX | Input Parameter Exchange |

### Invocation Mutations

| Operator | Description |
|-----------|-------------|
| IMCD | Invocation Mutation Conditional Deletion |

---

## Configuration

Mutantor accepts configuration directly through the attribute macro.

### Example

```rust
#[generate_mutants(
    AOR,
    ROR,
    path = "generated.rs",
    m_count = 3,
    c_count = 2,
    chance = 0.75,
    acc = 85
)]
```

### Options

| Option   | Description                    | Default |
|----------|--------------------------------|---------|
| path     | Write generated code to a file | None    |
| m_count  | Mutations per operator         | 1       |
| c_count  | Mutation combinations          | 3       |
| chance   | Mutation probability           | 0.75    |
| acc      | Required mutation score (%)    | 75      |
| use_acoc | Use ACOC based inputs          | false   |

---

## Statement Deletion

Mutantor provides the `sdl!` macro to mark statements that can be removed by SDL mutations.

```rust
use mutantor::sdl;

sdl! {
    println!("debug");
}
```

The mutation engine may replace this with:

```rust
sdl!();
```

during mutation testing.

---

## Ignored Statements

The `ignore!` macro can be used to mark statements that should be treated specially by mutation tooling.

```rust
use mutantor::ignore;

ignore! {
    println!("diagnostic output");
}
```

---

## Custom Types

Mutantor generates randomized inputs using the `Mutable` trait.

Example:

```rust
use mutantor::rand_val::Mutable;
use mutantor::rand::rngs::ThreadRng;

#[derive(Clone)]
struct User {
    id: u32,
}

impl Mutable for User {
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        Self {
            id: u32::new_mutable(rng),
        }
    }
}
```

---

## AI-Assisted Mutation Planning

Mutantor can automatically generate mutation strategies using an LLM.

```rust
#[generate_mutants_ai]
fn calculate(a: i32, b: i32) -> i32 {
    a + b
}
```

Create a `mutant.toml` file:

```toml
api_key = "YOUR_OPENROUTER_API_KEY"
```

The AI analyzes the function and automatically selects:

- Mutation operators
- Mutation counts
- Mutation combinations
- Mutation probabilities
- Mutation score thresholds

---

## Example Output

```text
mutation 0 killed _ AOR
mutation 1 survived _ ROR
mutation 2 killed _ IPVR

score 66.67
```

---

## Project Structure

| Module | Purpose |
|----------|----------|
| `flag_macros` | Mutation helper macros |
| `mutation_builder` | Attribute parsing |
| `mutation_collector` | Mutant generation |
| `mutation_operators` | Mutation operator definitions |
| `rand_val` | Random input generation |

---

## Roadmap

- Better AI mutation planning
- Coverage-aware mutation selection
- Mutation reports (HTML/JSON)
- Parallel mutant execution
- Cargo subcommand integration
- CI/CD reporting support

---

## License

Licensed under either:

- MIT License
- Apache License 2.0

at your option.