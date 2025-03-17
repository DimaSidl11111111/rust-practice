fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let data = [
        (24, 60),
        (15, 9),
        (15, 6),
        (140, 40),
        (24, 16),
        (100, 10),
        (120, 80),
        (80, 120),
        (100, 20),
        (37, 11),
        (120, 90),
    ];
    
    for (a, b) in data.iter() {
        println!("GCD of {} and {} is {}", a, b, gcd(*a, *b));
    }
}
