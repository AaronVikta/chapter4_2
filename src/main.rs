
// Here is how to define and use calculate_length function
// that has a reference to an object as a parameter
// instead of taking ownership of the value

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    let mut s2 =String::from("hello");
    
    println!("The length of '{}' is {}.", s1,len); //value of len is 5 here an
    
    change(&mut s2); //value of len is 12 after the function call but not before
    let len = calculate_length(&s2);
    println!("The length of '{}' is {}.", s2,len);
}

fn calculate_length(s:&String) -> usize{
//    s is a reference to a String 
    s.len()
}//Here, s goes out of scope. But because it does not have 
// have ownership it refers to, it is dropped



fn change (some_string: &mut String){
    some_string.push_str(", World");
    println!("some_string {}",some_string);
}

