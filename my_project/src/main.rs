fn calc_sum(nums: [i32; 5]) -> i32 {
    let mut sum = 0;
    for n in nums {
        sum += n;
    }
    sum
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    let result = calc_sum(nums);

    if result % 2 == 0 {
        println!("Day 6: sum {} is even", result);
    } else {
        println!("Day 6: sum {} is odd", result);
    }
}