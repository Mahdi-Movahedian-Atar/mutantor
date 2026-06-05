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
#[cfg(test)]
#[allow(unused)]
mod mutation_demo_test {
    use super::*;
    use mutantor::rand;
    use mutantor::Mutable;
    #[test]
    fn test() {
        let mut score = 0f64;
        let mut rng = rand::rng();
        let mut a = i32::new_mutable(&mut rng);
        let mut b = i32::new_mutable(&mut rng);
        let mut c = bool::new_mutable(&mut rng);
        let mut d = bool::new_mutable(&mut rng);
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_0(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 0usize, " ROR, IMCD,");
        } else {
            println!("mutation {} killed _ {}", 0usize, " ROR, IMCD,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_1(
            b.clone_mutable(),
            a.clone_mutable(),
            d.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 1usize, " IPEX, ROR,");
        } else {
            println!("mutation {} killed _ {}", 1usize, " IPEX, ROR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_2(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 2usize, " SOR, ROR,");
        } else {
            println!("mutation {} killed _ {}", 2usize, " SOR, ROR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_3(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 3usize, " AOR, SOR,");
        } else {
            println!("mutation {} killed _ {}", 3usize, " AOR, SOR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_4(
            a.clone_mutable(),
            i32::new_mutable(&mut rng).clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 4usize, " IPVR, IPVR,");
        } else {
            println!("mutation {} killed _ {}", 4usize, " IPVR, IPVR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_5(
            b.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            c.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 5usize, " IPEX, IPEX,");
        } else {
            println!("mutation {} killed _ {}", 5usize, " IPEX, IPEX,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_6(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 6usize, " COR, LOR,");
        } else {
            println!("mutation {} killed _ {}", 6usize, " COR, LOR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_7(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 7usize, " AOR, SOR,");
        } else {
            println!("mutation {} killed _ {}", 7usize, " AOR, SOR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_8(
            i32::new_mutable(&mut rng).clone_mutable(),
            b.clone_mutable(),
            bool::new_mutable(&mut rng).clone_mutable(),
            bool::new_mutable(&mut rng).clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 8usize, " IPVR, COR,");
        } else {
            println!("mutation {} killed _ {}", 8usize, " IPVR, COR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == Mutable::new_mutable(&mut rng))
        {
            println!("mutation {} survived _ {}", 9usize, " IMCD, IMCD,");
        } else {
            println!("mutation {} killed _ {}", 9usize, " IMCD, IMCD,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_10(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 10usize, " LOR, IPEX,");
        } else {
            println!("mutation {} killed _ {}", 10usize, " LOR, IPEX,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_11(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 11usize, " IMCD, LOR,");
        } else {
            println!("mutation {} killed _ {}", 11usize, " IMCD, LOR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_12(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 12usize, " LOR, AOR,");
        } else {
            println!("mutation {} killed _ {}", 12usize, " LOR, AOR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_13(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 13usize, " SOR, COR,");
        } else {
            println!("mutation {} killed _ {}", 13usize, " SOR, COR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_14(
            a.clone_mutable(),
            i32::new_mutable(&mut rng).clone_mutable(),
            bool::new_mutable(&mut rng).clone_mutable(),
            bool::new_mutable(&mut rng).clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 14usize, " COR, IPVR,");
        } else {
            println!("mutation {} killed _ {}", 14usize, " COR, IPVR,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_15(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 15usize, " ROR, AOR,");
        } else {
            println!("mutation {} killed _ {}", 15usize, " ROR, AOR,");
            score += 1f64;
        }
        println!("score {}", (score / 16f64) * 100f64);
        assert!((score / 16f64) * 100f64 >= 100f64)
    }
    fn mutation_demo_0(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x <= 10 && c) || (x < 100 && d) {
            x += 1;
        }
        let flags = (a & b) | (a ^ b);
        let shifted = (flags << 2) >> 1;
        let tmp1 = shifted + 1;
        let tmp2 = shifted - 1;
        if x == tmp1 || x != tmp2 {
            x += 5;
        }
        if x < 0 && (true || true) {
            x *= 2;
        }
        x
    }
    fn mutation_demo_1(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x > 10 && c) || (x > 100 && d) {
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
    fn mutation_demo_2(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x >= 10 && c) || (x <= 100 && d) {
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
    fn mutation_demo_3(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a / b;
        x = x - 1;
        x = x * 2;
        x = x % 2;
        x = x % 7;
        if (x > 10 && c) || (x <= 100 && d) {
            x %= 1;
        }
        let flags = (a & b) | (a ^ b);
        let shifted = (flags << 2) >> 1;
        let tmp1 = shifted % 1;
        let tmp2 = shifted + 1;
        if x == tmp1 || x != tmp2 {
            x *= 5;
        }
        if x >= 0 && x < 1000 {
            x *= 2;
        }
        x
    }
    fn mutation_demo_4(a: i32, b: i32, c: bool, d: bool) -> i32 {
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
    fn mutation_demo_5(a: i32, b: i32, c: bool, d: bool) -> i32 {
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
    fn mutation_demo_6(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x > 10 || c) || (x <= 100 && d) {
            x += 1;
        }
        let flags = (a | b) ^ (a | b);
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
    fn mutation_demo_7(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x % 1;
        x = x % 2;
        x = x * 2;
        x = x % 7;
        if (x > 10 && c) || (x <= 100 && d) {
            x /= 1;
        }
        let flags = (a & b) | (a ^ b);
        let shifted = (flags >> 2) >> 1;
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
    fn mutation_demo_8(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x > 10 && c) || (x <= 100 || d) {
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
    fn mutation_demo_10(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x > 10 && c) || (x <= 100 && d) {
            x += 1;
        }
        let flags = (a ^ b) | (a & b);
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
    fn mutation_demo_11(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (x > 10 && c) || (x <= 100 && d) {
            x += 1;
        }
        let flags = (a & b) | (a & b);
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
    fn mutation_demo_12(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a % b;
        x = x - 1;
        x = x % 2;
        x = x % 2;
        x = x % 7;
        if (x > 10 && c) || (x <= 100 && d) {
            x *= 1;
        }
        let flags = (a & b) | (a ^ b);
        let shifted = (flags << 2) >> 1;
        let tmp1 = shifted + 1;
        let tmp2 = shifted - 1;
        if x == tmp1 || x != tmp2 {
            x += 5;
        }
        if x >= 0 && x < 1000 {
            x -= 2;
        }
        x
    }
    fn mutation_demo_13(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (true || true) || (x <= 100 && d) {
            x += 1;
        }
        let flags = (a & b) | (a ^ b);
        let shifted = (flags >> 2) << 1;
        let tmp1 = shifted + 1;
        let tmp2 = shifted - 1;
        if false || false {
            x += 5;
        }
        if true || true {
            x *= 2;
        }
        x
    }
    fn mutation_demo_14(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        x = x % 7;
        if (false || false) || (x <= 100 && d) {
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
    fn mutation_demo_15(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a * b;
        x = x / 1;
        x = x + 2;
        x = x / 2;
        x = x / 7;
        if (x > 10 && c) || (x < 100 && d) {
            x += 1;
        }
        let flags = (a & b) | (a ^ b);
        let shifted = (flags << 2) >> 1;
        let tmp1 = shifted % 1;
        let tmp2 = shifted - 1;
        if x == tmp1 || x != tmp2 {
            x += 5;
        }
        if x >= 0 && x < 1000 {
            x %= 2;
        }
        x
    }
}
