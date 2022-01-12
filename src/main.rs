// Description: 
// Method1 : function palindrome 
// step 1 - input 
// step 2 - call function reverse input (return string)
// step 3 - compare between input value and reverse value (return boolean)
// * Note : if input == reverse input > 'true', else input != reverse input > false

fn main() {
    // put the values on this line to test 
    println!("{}", is_palindrome_method_1("121"));
}

fn is_palindrome(input: &str) -> bool {
    input == reverse_input(input)
}

// reverse input
fn reverse_input(input: &str) -> String {
    input.chars().rev().collect()
}
