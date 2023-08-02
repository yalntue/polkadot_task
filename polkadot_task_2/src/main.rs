// Define a struct for filtering conditions.
struct FilterCondition {
    value: i32,
    is_even: bool,
}

// Implement an is_match method on the FilterCondition struct.
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        if self.is_even {
            *item % 2 == 0
        } else {
            *item % 2 != 0
        }
    }
}

// Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments.
// The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_items = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            filtered_items.push(*item);
        }
    }

    filtered_items
}

fn main() {
    // Create a collection (e.g., a vector) and add some elements to it
    let numbers = vec![10, 21, 30, 45, 50, 63, 70, 89, 92, 101];

    // Initialize a FilterCondition object with the desired value
    let filter_condition_even = FilterCondition { value: 0, is_even: true };
    let filter_condition_odd = FilterCondition { value: 0, is_even: false };

    // Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable
    let filtered_even_numbers = custom_filter(&numbers, &filter_condition_even);
    let filtered_odd_numbers = custom_filter(&numbers, &filter_condition_odd);

    // Print the filtered result to the console
    println!("Filtered Even Numbers: {:?}", filtered_even_numbers);
    println!("Filtered Odd Numbers: {:?}", filtered_odd_numbers);
}
