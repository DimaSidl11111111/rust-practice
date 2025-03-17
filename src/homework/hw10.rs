fn is_palindrome(n: i32) -> bool {
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main() {
    let numbers = [123, 121, 1221];

    numbers.iter().for_each(|&n| {
        let result = is_palindrome(n);
        println!("Number: {}, Is palindrome: {}", n, result);
    });
}
