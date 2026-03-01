fn main() {
    let nums = vec![2, 4, 6, 8];
    let mut sum = 0;

    for (i, n) in nums.iter().enumerate() {
        println!("index {} value {}", i, n);
        sum += n;
    }

    println!("Day 7: sum = {}", sum);
}