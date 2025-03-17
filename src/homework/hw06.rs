fn draw_tree(triangles: usize) {
    let mut height = 1;
    let max_width = (2 * triangles - 1) * 2 - 1;

    for _ in 0..triangles {
        for i in 0..height {
            let spaces = " ".repeat((max_width - (2 * i + 1)) / 2);
            let stars = "*".repeat(2 * i + 1);
            println!("{}{}", spaces, stars);
        }
        height += 1;
    }

    let trunk_width = 3;
    let trunk_space = " ".repeat((max_width - trunk_width) / 2);
    for _ in 0..(triangles) {
        println!("{}{}", trunk_space, "*".repeat(trunk_width));
    }
}

fn main() {
    let triangles = 5;
    draw_tree(triangles);
}
