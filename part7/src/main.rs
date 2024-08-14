fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    // Borrows the numbers vec for each of the functions
    let sum_of_nums = sum(&numbers);
    let product_of_nums = product(&numbers);
    let average_of_nums = average(&numbers);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);

    let other_numbers = vec![1, 2, 3, 4, 5, 6];
    // Set slice 1 and slice 2 both equal to references to a vec
    // Borrow the two different vecs(numbers1, numbers2) here inside the first_three function
    let (slice1, slice2) = first_three(&numbers, &other_numbers);

    println!("The first three elements in `slice1` are:");

    for num in slice1 {
        println!("• {}", num);
    }

    println!("The first three elements in `slice2` are:");

    for num in slice2 {
        println!("• {}", num);
    }
}

// Set numbers equal to a slice of the vec
fn sum(numbers: &[i64]) -> i64 {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

fn product(numbers: &[i64]) -> i64 {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    total
}

fn average(numbers: &[i64]) -> i64 {
    let length = numbers.len() as i64;

    sum(numbers) / length
}

// This sets up the lifetime annotation. numbers1 has a lifetime annotation of a and numbers2 has a lifetime annotation of b
// Then you call a reference to the vec and attach the annotations to each
// Then you would get a return a slice of a vec with each separate lifetime annotation
fn first_three<'a, 'b>(numbers1: &'a Vec<i64>, numbers2: &'b Vec<i64>) -> (&'a [i64], &'b [i64]) {
    (&numbers1[0..3], &numbers2[0..3])
    // This borrows from the numbers1 and numbers2 reference, and pulls the first three values from the vec
}
