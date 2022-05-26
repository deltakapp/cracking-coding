// Problem: Implement an algorithm to determine if a string has all unique
// characters. What if you can't use additional data structures?

// Returns true if all characters are unique in given string
pub fn main(string: String) -> bool {
    let chars = string.chars();
    let mut unique: bool = true; // if characters are all unique or not

    let mut i = 0; // index of char in string

    for char in chars {
        // check the remainder of string for duplicate chars
        if string[(i + 1)..].contains(char) {
            unique = false;
            break;
        }
        i = i + 1;
    }
    unique
}
