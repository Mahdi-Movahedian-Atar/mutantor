fn deletion_demo(a: bool, b: bool) -> bool {
    (a && b) || (!a && !b)
}
#[cfg(test)]
#[allow(unused)]
mod deletion_demo_test {
    use super::*;
    use mutantor::rand;
    use mutantor::Mutable;
    #[test]
    fn test() {
        let mut score = 0f64;
        let mut rng = rand::rng();
        let mut a = bool::new_mutable(&mut rng);
        let mut b = bool::new_mutable(&mut rng);
        if (deletion_demo(a.clone_mutable(), b.clone_mutable())
            == deletion_demo_0(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 0usize, " LOD, COD,");
        } else {
            println!("mutation {} killed _ {}", 0usize, " LOD, COD,");
            score += 1f64;
        }
        if (deletion_demo(a.clone_mutable(), b.clone_mutable())
            == deletion_demo_1(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 1usize, " COD, COD,");
        } else {
            println!("mutation {} killed _ {}", 1usize, " COD, COD,");
            score += 1f64;
        }
        if (deletion_demo(a.clone_mutable(), b.clone_mutable())
            == deletion_demo_2(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 2usize, " LOD, LOD,");
        } else {
            println!("mutation {} killed _ {}", 2usize, " LOD, LOD,");
            score += 1f64;
        }
        if (deletion_demo(a.clone_mutable(), b.clone_mutable())
            == deletion_demo_3(a.clone_mutable(), b.clone_mutable()))
        {
            println!("mutation {} survived _ {}", 3usize, " COD, LOD,");
        } else {
            println!("mutation {} killed _ {}", 3usize, " COD, LOD,");
            score += 1f64;
        }
        println!("score {}", (score / 4f64) * 100f64);
        assert!((score / 4f64) * 100f64 >= 100f64)
    }
    fn deletion_demo_0(a: bool, b: bool) -> bool {
        (b && b) || (a && b)
    }
    fn deletion_demo_1(a: bool, b: bool) -> bool {
        (a && b) || (b && b)
    }
    fn deletion_demo_2(a: bool, b: bool) -> bool {
        (a && b) || (!a && !b)
    }
    fn deletion_demo_3(a: bool, b: bool) -> bool {
        (a && a) || (b && b)
    }
}
