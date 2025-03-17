use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = (0, 1);
    
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = (i, i + 1);
        }
    }
    (min_sum, min_index.0, min_index.1)
}

fn print_vector_info(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:<3}", i)).collect::<String>());
    println!("data:   [{}]", data.iter().map(|&x| format!("{:<3}", x)).collect::<String>());
    
    let (min_sum, i1, i2) = min_adjacent_sum(data);
    
    let indexes_str: String = (0..data.len())
        .map(|i| if i == i1 || i == i2 { "__" } else { "   " })
        .collect();
    
    println!("indexes:{}", indexes_str);
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[i1], data[i2], min_sum, i1, i2);
}

fn main() {
    let data1 = gen_random_vector(20);
    let data2 = gen_random_vector(20);
    let data3 = gen_random_vector(20);
    let data4 = gen_random_vector(20);
    
    println!("\nFirst random vector:");
    print_vector_info(&data1);
    
    println!("\nSecond random vector:");
    print_vector_info(&data2);
    
    println!("\nThird random vector:");
    print_vector_info(&data3);
    
    println!("\nFourth random vector:");
    print_vector_info(&data4);
}
