use core::num;

fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    // This is setting up a new variable with an (i64, Vec<i64>), which is why you need sum_of, product_of AND the new numbers variable
    // You set the tuple to the sum_of(establishing the i64 variable here) and numbers2(establishing the new numbers variable with OWNERSHIP)
    let (sum_of_nums, numbers2) = sum(numbers);
    // Set the product_of variable to an i64, and numbers3 with the OWNERSHIP of numbers2 from the product function call
    let (product_of_nums, numbers3) = product(numbers2);
    // Carry down the OWNERSHIP from numbers3 into the function of average since you don't need another Vec after this, it gets removed once it is called below
    let average_of_nums = average(numbers3);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

fn sum(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    (total, numbers)
}

fn product(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    (total, numbers)
}

fn average(numbers: Vec<i64>) -> i64 {
    let length = numbers.len() as i64;

    sum(numbers).0 / length
    // This works because you'd be pulling the i64 out of the tuple with the .0
}
