#![allow(dead_code)]

mod combination_lock; // Combination lock mini project
mod control_flow; // If, While, For and Match statements
mod data_structures; // Structs, Impl, Enums, Option<T>, Type and Union
mod pattern_matching; // Match statements to match partterns
mod ownership_borrowing; // Ownership and borrowing
mod types_variables; // Data types, variable types and generics
mod standard_collections; // Vectors, HashMaps, HashSet and Iterators
mod chars_strings; // Chars and different strings and slices
mod functions; // Functions within functions
mod traits; // Traits, trait bound, supertraits, deriving
mod lifetimes; // Lifetimes
mod pointers; // Box smart pointer, rc smart pointer, refcell smart pointer
mod error_handling; // Error handling
mod dep_error_handling; // Error handling with third party error handling (main fn)
mod hash_error_handling; // Using error-stack for error handling

fn main() {
    hash_error_handling::main()
}