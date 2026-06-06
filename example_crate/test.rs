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
            println!("mutation {} survived _ {}", 0usize, " IPEX, IMCD,");
        } else {
            println!("mutation {} killed _ {}", 0usize, " IPEX, IMCD,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_1(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 1usize, " LOR, SOR,");
        } else {
            println!("mutation {} killed _ {}", 1usize, " LOR, SOR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_2(
                i32::new_mutable(&mut rng).clone_mutable(),
                bool::new_mutable(&mut rng).clone_mutable(),
            ))
        {
            println!("mutation {} survived _ {}", 2usize, " SDL, IPVR,");
        } else {
            println!("mutation {} killed _ {}", 2usize, " SDL, IPVR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_3(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 3usize, " SOR, IPEX,");
        } else {
            println!("mutation {} killed _ {}", 3usize, " SOR, IPEX,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_4(
                a.clone_mutable(),
                bool::new_mutable(&mut rng).clone_mutable(),
            ))
        {
            println!("mutation {} survived _ {}", 4usize, " IPVR, AOR,");
        } else {
            println!("mutation {} killed _ {}", 4usize, " IPVR, AOR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_5(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 5usize, " ROR, COR,");
        } else {
            println!("mutation {} killed _ {}", 5usize, " ROR, COR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_6(
                i32::new_mutable(&mut rng).clone_mutable(),
                bool::new_mutable(&mut rng).clone_mutable(),
            ))
        {
            println!("mutation {} survived _ {}", 6usize, " IPVR, COR,");
        } else {
            println!("mutation {} killed _ {}", 6usize, " IPVR, COR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_7(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 7usize, " LOR, SDL,");
        } else {
            println!("mutation {} killed _ {}", 7usize, " LOR, SDL,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_8(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 8usize, " AOR, AOR,");
        } else {
            println!("mutation {} killed _ {}", 8usize, " AOR, AOR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable()) == Mutable::new_mutable(&mut rng))
        {
            println!("mutation {} survived _ {}", 9usize, " IMCD, ROR,");
        } else {
            println!("mutation {} killed _ {}", 9usize, " IMCD, ROR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_10(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 10usize, " COR, LOR,");
        } else {
            println!("mutation {} killed _ {}", 10usize, " COR, LOR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_11(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 11usize, " SOR, IMCD,");
        } else {
            println!("mutation {} killed _ {}", 11usize, " SOR, IMCD,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_12(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 12usize, " COR, SDL,");
        } else {
            println!("mutation {} killed _ {}", 12usize, " COR, SDL,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_13(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 13usize, " IPEX, ROR,");
        } else {
            println!("mutation {} killed _ {}", 13usize, " IPEX, ROR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_14(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 14usize, " ROR, LOR,");
        } else {
            println!("mutation {} killed _ {}", 14usize, " ROR, LOR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_15(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 15usize, " SDL, SOR,");
        } else {
            println!("mutation {} killed _ {}", 15usize, " SDL, SOR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable()) == Mutable::new_mutable(&mut rng))
        {
            println!("mutation {} survived _ {}", 16usize, " IMCD, IPVR,");
        } else {
            println!("mutation {} killed _ {}", 16usize, " IMCD, IPVR,");
            score += 1f64;
        }
        if (insertion_demo(a.clone_mutable(), b.clone_mutable())
            == insertion_demo_17(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 17usize, " AOR, IPEX,");
        } else {
            println!("mutation {} killed _ {}", 17usize, " AOR, IPEX,");
            score += 1f64;
        }
        println!("score {}", (score / 18f64) * 100f64);
        assert!((score / 18f64) * 100f64 >= 100f64)
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
    fn insertion_demo_6(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_7(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_8(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_10(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_11(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_12(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_13(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_14(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_15(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
    fn insertion_demo_17(a: i32, b: bool) -> i32 {
        let x = a;
        if b {
            1
        } else {
            0
        }
    }
}
