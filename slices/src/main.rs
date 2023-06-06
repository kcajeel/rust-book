/*
Here’s a small programming problem: write a function that takes a string of words separated 
by spaces and returns the first word it finds in that string. If the function doesn’t find 
a space in the string, the whole string must be one word, so the entire string should be 
returned.
*/

fn main() {
    let str1 = String::from("Hello world");
    let str2 = String::from("world Hello");
    print!("{}", &problem(&str1));
    print!(", {}!", &problem(&str2));
}

fn problem(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
