// Description: 
// Method1 : function palindrome 
// step 1 - input 
// step 2 - call function reverse input (return string)
// step 3 - compare between input value and reverse value (return boolean)
// for homework 3 : step 4 - check word ("end" or "End" or "END")for end process
// * Note : if input == reverse input > 'true', else input != reverse input > false

use std::process;

fn main() {
    // put the values on this line to test 
    println!("input = {}, result = {}", "121", is_palindrome("121"));
    println!("input = {}, result = {}", "Anna", is_palindrome("Anna"));
    println!("input = {}, result = {}", "end", is_palindrome("END"));
}

// function: check palindrome
fn is_palindrome(input: &str) -> bool {
    end_process(input);
    input == reverse_input(input)
}

// function: reverse input
fn reverse_input(input: &str) -> String {
    input.chars().rev().collect()
}

// function: check end process
fn end_process(input: &str) {
    let new_input = input.to_lowercase();
    if new_input == "end" {
        process::exit(1);
    }
}