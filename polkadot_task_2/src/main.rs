// Define a struct called FilterCondition with a single field of the desired type for filtering.
struct FilterCondition<T> {
    condition: T,
}

// Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

// Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments.
// The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
fn custom_filter<T>(collection: &Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    collection
        .iter()
        .cloned()
        .filter(|item| filter_condition.is_match(item))
        .collect()
}

fn main() {
    // Create a collection (e.g., a vector) with some elements
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Initialize a FilterCondition object with the desired value
    let filter_condition = FilterCondition { condition: 5 };

    // Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable
    let filtered_numbers = custom_filter(&numbers, &filter_condition);

    // Print the filtered result to the console
    println!("Filtered Numbers: {:?}", filtered_numbers);
}
