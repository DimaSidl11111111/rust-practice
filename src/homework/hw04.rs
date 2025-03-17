const SIZE: usize = 6; 
fn main() {
    let mut output = String::new();
    
    for i in 0..(2 * SIZE - 1) {
        let stars = if i < SIZE { 2 * i + 1 } else { 2 * (2 * SIZE - 2 - i) + 1 };
        let spaces = (2 * SIZE - 1 - stars) / 2;
        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(stars));
        output.push('\n');
    }
    
    print!("{}", output);
}
