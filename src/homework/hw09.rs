fn rotate2(s: &str, n: i32) -> String {
    let len = s.len();
    if len == 0 {
        return s.to_string();
    }

    let n = n % len as i32;

    let n = if n < 0 {
        (n + len as i32) % len as i32
    } else {
        n
    };

    let n = n as usize;
    let (left, right) = s.split_at(len - n);

    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        let result = rotate2(s, *n);
        println!("Shift by {}: {}", n, result);
        assert_eq!(result, exp.to_string());
    });
}
