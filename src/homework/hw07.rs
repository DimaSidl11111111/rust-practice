fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
}

fn main() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];

    data.iter().for_each(|(a, b)| {
        let result_a = invert_the_case(a.to_string());
        let result_b = invert_the_case(b.to_string());

        println!("Original: {}, Inverted: {}", a, result_a);
        println!("Original: {}, Inverted: {}", b, result_b);
    });
}
