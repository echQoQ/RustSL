#[cfg(feature = "vm_check_prime")]
pub fn check_prime() -> bool {
    let n1: u64 = 1_000_000_000_000_002_493;
    for _ in 0..30 {
        if n1 <= 1 {
            return false;
        }
        let mut is_prime = true;
        let sqrt_n = (n1 as f64).sqrt() as u64 + 1;
        for i in 2..sqrt_n {
            if n1 % i == 0 {
                is_prime = false;
                break;
            }
        }
        if !is_prime {
            return false;
        }
    }
    true
}
