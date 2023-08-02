# Polkadot-Rust-Bootcamp-Week-3

Polkadot Rust Bootcamp Third Week Assignment:

Hands-on Task - Building a Custom Filtering Function in Rust

Create a custom filtering function in Rust that allows filtering elements from a given collection based on a specific condition. 
Goal: The goal is to implement a beginner-friendly solution that avoids using closures to simplify the understanding of the code.

Steps:

1. Define a struct called FilterCondition with a single field of the desired type for filtering.

2. Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.

3. Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.

4. In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.

5. Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.

6. Print the filtered result to the console.

7. Compile and run the program to test its functionality.
