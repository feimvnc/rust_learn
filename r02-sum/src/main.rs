// Working code
// fn calculate_sum(numbers: &[i32]) -> Result<i32, String> {
//     let mut sum = 0;
//     // for loop used, but not iterator adaptor
//     for num in numbers.iter().cloned() {
//         sum += num;
//     }
//     Ok(sum)
// }

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];
//     // error case not handled, but should be
//     let sum = calculate_sum(&numbers).unwrap();
//     println!("Sum: {}", sum)
// }

// Right code
// fn calculate_sum(numbers: &[i32]) -> Result<i32, String> {
//     let sum = numbers.iter().sum();
//     Ok(sum)
// }

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];
//     match calculate_sum(&numbers) {
//         Ok(sum) => println!("Sum: {}", sum),
//         Err(err) => eprintln!("Error: {}", err),
//     }
// }

// Fast code using parallel iterators
use rayon::prelude::*;

fn calculate_sum(numbers: &[i32]) -> Result<i32, String> {
    let sum = numbers.par_iter().sum();
    Ok(sum)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match calculate_sum(&numbers) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(err) => eprintln!("Error: {}", err),
    }
}
