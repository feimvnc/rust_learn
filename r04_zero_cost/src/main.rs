/* C++ code

// v1
int main() {
    std::vector<int> numbers = {5,2,8,1,3};

    // Manual implementation to find th maximum element
    // No abstraction, has to do loop, comparison, and keep track of the maximum value
    // Less readable and more error-prone
    int maxElement = numbers[0];
    for (size_t i = 1; i < numbers.size(); ++i) {
        if (numbers[i] > maxElement) {
            maxElement = numbers[i];
        }
    }

    std::cout << "Maximum element: " << maxElement << std::endl;

    return 0;
}

// v2 use iterator abstraction using iteration and comparison logic
int main() {
    std::vector<int> numbers = {5,2,3,1,3};

    // Using iterator abstraction to find the maximum elelemt
    auto maxElement = std::max_element(numbers.begin(), numbers.end());

    std::cout << "Maximum element: " << *maxElement << std::endl;

    return 0;
}

// Python iterator is not a zero-cost abstraction, it has overhead and is not as efficient as C++ iterators.

numbers = [5,2,8,1,3]
# Using iterator abstraction to find the maximum element
max_element = max(numbers, default=None)
if max_element is not None:
    print(f"Maximum element: {max_element}")
else:
    print("The list is empty.")
*/

// rust
fn main() {
    let numbers = [8, 2, 1, 3, 2];
    //Using iterator abstraction to find the maximum element
    let max_element = numbers.iter().max();

    // Pattern matching to handle the Option returned by max()
    match max_element {
        Some(&max) => println!("Max element: {}", max),
        None => println!("The vector is empty."),
    }
}
