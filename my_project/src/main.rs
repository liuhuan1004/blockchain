fn main() {
    let nums = [1, 2, 3, 4, 5];
    let mut sum = 0;

    for n in nums {
        sum += n;
    }

    if sum > 10 {
        println!("Day 5: sum is big: {}", sum);
    } else {
        println!("Day 5: sum is small: {}", sum);
    }
}