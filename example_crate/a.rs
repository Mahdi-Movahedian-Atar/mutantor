fn mutation_demo(a: i32, b: i32, c: bool, d: bool) -> i32 {
    let mut x = a + b;
    x = x - 1;
    x = x * 2;
    x = x / 2;
    sdl!(x = x % 7);
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
            println!("mutation {} survived _ {}", 0usize, " SDL, SDL,");
        } else {
            println!("mutation {} killed _ {}", 0usize, " SDL, SDL,");
            score += 1f64;
        }
        if (mutation_demo(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        ) == mutation_demo_1(
            a.clone_mutable(),
            b.clone_mutable(),
            c.clone_mutable(),
            d.clone_mutable(),
        )) {
            println!("mutation {} survived _ {}", 1usize, " SDL, SDL,");
        } else {
            println!("mutation {} killed _ {}", 1usize, " SDL, SDL,");
            score += 1f64;
        }
        println!("score {}", (score / 2f64) * 100f64);
        assert!((score / 2f64) * 100f64 >= 100f64)
    }
    fn mutation_demo_0(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        sdl!();
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
    fn mutation_demo_1(a: i32, b: i32, c: bool, d: bool) -> i32 {
        let mut x = a + b;
        x = x - 1;
        x = x * 2;
        x = x / 2;
        sdl!();
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
}
