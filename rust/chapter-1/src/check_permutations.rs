// Problem: given two strings, write a method to check if one is a permutation
// of the other

// Solution
use itertools::Itertools;

pub fn main(string_1: String, string_2: String) -> bool {
    //sort both strings
    let sorted_string_1 = string_1.chars().sorted().collect::<String>();
    let sorted_string_2 = string_2.chars().sorted().collect::<String>();
    let is_permutation = sorted_string_1 == sorted_string_2; // compare strings
    is_permutation
}
