fn insertion_demo(a: i32, b: bool) -> i32 {
    let x = a;
    if b {
        1
    } else {
        0
    }
}
#[cfg(test)]
#[allow(unused)]
mod insertion_demo_test {
    use super::*;
    use mutantor::rand;
    use mutantor::Mutable;
    #[test]
    fn test() {
        let mut score = 0f64;
        let mut rng = rand::rng();
        let mut a = i32::new_mutable(&mut rng);
        let mut b = bool::new_mutable(&mut rng);
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_0(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 0usize, " LOI, AOI,");
        } else {
            println!("mutation {} killed _ {}", 0usize, " LOI, AOI,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_1(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 1usize, " COI, COI,");
        } else {
            println!("mutation {} killed _ {}", 1usize, " COI, COI,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_2(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 2usize, " LOI, COI,");
        } else {
            println!("mutation {} killed _ {}", 2usize, " LOI, COI,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_3(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 3usize, " COI, LOI,");
        } else {
            println!("mutation {} killed _ {}", 3usize, " COI, LOI,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_4(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 4usize, " AOI, LOI,");
        } else {
            println!("mutation {} killed _ {}", 4usize, " AOI, LOI,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_5(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 5usize, " AOI, AOI,");
        } else {
            println!("mutation {} killed _ {}", 5usize, " AOI, AOI,");
            score += 1f64;
        }
        println!("score {}", (score / 6f64) * 100f64);
        assert!((score / 6f64) * 100f64 >= 100f64)
    }
    fn insertion_demo_0(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_1(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_2(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_3(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_4(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_5(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
}
