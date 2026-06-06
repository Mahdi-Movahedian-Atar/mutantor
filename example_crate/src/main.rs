use mutantor::*;


fn main() {
    println!("Hello, world!");
}

#[generate_mutants(IPVR, IPEX, IMCD,AOR,ROR,COR,LOR,SOR,SDL, path = "a.rs" , acc = 100.0, c_count = 2,m_count = 2)]
fn mutation_demo(a: i32, b: i32, c: bool, d: bool) -> i32 {
    let mut x = a + b;
    x = x - 1;
    x = x * 2;
    x = x / 2;
    x = x % 7;


    if (x > 10 && c) || (x <= 100 && d) {
        x += 1;
    }

    let flags = (a & b) | (a ^ b);

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
#[generate_mutants(AOI, COI, LOI, path = "b.rs" , acc = 100.0, c_count = 2,m_count = 2)]
fn insertion_demo(a: i32, b: bool) -> i32 {
    let x = a;

    if b {
        1
    } else {
        0
    }
}
#[generate_mutants(COD,LOD, path = "c.rs" , acc = 100.0, c_count = 2,m_count = 2)]
fn deletion_demo(a: bool, b: bool) -> bool {
    (a && b) || (!a && !b)
}