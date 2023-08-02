// TASK: Building a Custom Filtering Function in Rust
// Limitation: avoid using closures to simplify the understanding of the code

fn main() {
    // Create a collection (e.g., a vector) with some elements
    let collection = vec![9, 7, 2, 12, 13, 5, 8, 23, 4, 6, 11, 1];

    // Initialize a FilterCondition object with the desired value
    let filter_condition = FilterCondition { condition: 1 };

    // Call the custom_filter function with the collection and the FilterCondition object and storing the result in a new variable
    let filtered_result = custom_filter(collection, &filter_condition);

    // Print the filtered result to the console
    println!("Result is: {:?}", filtered_result);
}

// Define a struct called FilterCondition with a single field of the desired type for filtering
struct FilterCondition<T> {
    condition: T
}

// Implement a method called is_match on the FilterCondition struct that 
// takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T)  -> bool {
        *item == self.condition
    }  
}

// Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. 
// The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
fn custom_filter<T>(collection: Vec<T>, filtered_condition: &FilterCondition<T>) -> Vec<T> 
where
    T: PartialEq,
{
    let mut filtered_collection = Vec::new();

    for item in collection {
        if filtered_condition.is_match(&item) {
            filtered_collection.push(item);
        }
    }

    filtered_collection
}
