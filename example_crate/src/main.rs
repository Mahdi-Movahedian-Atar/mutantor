use mutantor::generate_mutants;

fn main() {
    println!("Hello, world!");
}

#[generate_mutants(IPVR, IPEX, IMCD,AOR,ROR,COR,LOR,SOR, path = "test.rs" , acc = 100.0, c_count = 2,m_count = 2)]
fn mutation_demo(a: i32, b: i32, c: bool, d: bool) -> i32 {
    // AOR
    let mut x = a + b;
    x = x - 1;
    x = x * 2;
    x = x / 2;
    x = x % 7;



    // ROR + COR
    if (x > 10 && c) || (x <= 100 && d) {
        x += 1;
    }

    // LOR
    let flags = (a & b) | (a ^ b);

    // SOR
    let shifted = (flags << 2) >> 1;


    // SDL opportunities
    let tmp1 = shifted + 1;
    let tmp2 = shifted - 1;

    // More COR/ROR
    if x == tmp1 || x != tmp2 {
        x += 5;
    }

    if x >= 0 && x < 1000 {
        x *= 2;
    }

    x
}