fn main() {

// Booleans
let _my_first_bool: bool = true;
let _my_second_bool: bool = false;

//Integers
// 8, 16, 32, 64, 128,
let _days_of_week: i8 = 7;
let _number_of_users: i64 = 128000;

let _number_of_tokens: u64 = 10000;

let just_a_number = 0;

// floating point
// 32 or 64 -bit 
let pi: f32 = 3.14159;

// Characters
let _my_first_char: char = 'a';

// Strings
let _my_first_string: &str = "Hello, World!";
let _my_string: String = String::from("Hello, World!");

// Arrays
let _days_of_the_week: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

let first_element: &str = _days_of_the_week[0];
let last_element: &str = _days_of_the_week[_days_of_the_week.len() - 1];

//Slices
let slice: &[&str] = &_days_of_the_week[0..3];
let first_element_of_slice: &str = slice[0];

// Tuples
let person: (&str, i32) = (
    "Alice" , 30
);

let name: &str = person.0;
let age: i32 = person.1; 

// Unit type
let unit_type = ();

// Variables
let mut num: i32 = 5;
num = 6;
}

