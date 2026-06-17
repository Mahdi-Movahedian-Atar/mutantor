use mutantor::acoc;
use mutantor::rand::prelude::ThreadRng;
use mutantor::sdl;
use mutantor::*;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, PartialEq, PartialOrd)]
struct TestInput {
    a: i32,
}
impl Mutable for TestInput {
    fn new_mutable(rng: &mut ThreadRng) -> Self {
        Self {
            a: i32::new_mutable(rng),
        }
    }

    fn acoc(input: &[Self], rng: &mut ThreadRng) -> Vec<Self> {
        if input.len() == 0 {
            return vec![Self::new_mutable(rng)];
        }
        let mut out = Vec::from(input);
        for i in input {
            let a =TestInput{a:i.a-1};
            let b = TestInput{a:i.a+1};
            out.push(a);
            out.push(b);
        }
        out
    }
}

#[generate_mutants(
    IPVR,
    IPEX,
    IMCD,
    AOR,
    ROR,
    COR,
    LOR,
    SOR,
    SDL,
    use_acoc,
    path = "example_crate//a.rs",
    acc = 100.0,
    c_count = 2,
    m_count = 2,
    chance = 1.0
)]
fn mutation_demo(a: TestInput, b: i32, c: bool, d: bool) -> i32 {
    let mut x = a.a + b;
    x = x - 1;
    x = x * 2;
    x = x / 2;
    sdl!(x = x % 7);

    if (a > acoc!(TestInput { a: 10 }) && c) || (x <= 100 && d) {
        x += 1;
    }

    let flags = (a.a & b) | (a.a ^ b);

    let shifted = (flags << 2) >> 1;

    let tmp1 = shifted + 1;
    let tmp2 = shifted - 1;

    if x == tmp1 || x != tmp2 {
        x += 5;
    }

    if x >= 0 && x < 1000 {
        x *= 2;
    }

    x
}
#[generate_mutants(
    AOI,
    COI,
    LOI,
    use_acoc,
    path = "example_crate//b.rs",
    acc = 100.0,
    c_count = 2,
    m_count = 2
)]
fn insertion_demo(a: i32, b: bool) -> i32 {
    let x = a;

    if b == false { 1 } else { 0 }
}
#[generate_mutants(
    COD,
    LOD,
    path = "example_crate/c.rs",
    acc = 100.0,
    c_count = 2,
    m_count = 2
)]
fn deletion_demo(a: bool, b: bool) -> bool {
    (a && b) || (!a && !b)
}
