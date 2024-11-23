fn main() {
    explain_scope();
    explain_move();
    explain_move_calling_function();
    explain_tedious_moving();
    explain_reference_borrowing();
    explain_mutable_reference();
    explain_only_one_mutable_reference();
    explain_slice();
}

fn explain_slice() {
    //write a function that takes a string of words
    //separated by spaces and returns the first word
    //it finds in that string.
    //If the function doesn’t find a space in the string,
    //the whole string must be one word,
    //so the entire string should be returned
    let s = String::from("hello world");
    let value = first_word(&s);
    println!("first word is {value}");

    let value = first_word1(&s);
    println!("first word is {value}");
}

//We can also change the parameter to String slice type
fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//String slice is written as `&str`
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn explain_only_one_mutable_reference() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    //we can't have this below. It is a restriction of mutable reference.
    // let s2 = &mut s;
    // println!("{s1}, {}", &mut s);
    println!("{s1}");
}

fn explain_mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s after changed is: '{s}'");
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn explain_reference_borrowing() {
    let s1 = String::from("hello");
    //using a reference of s1 is called borrowing in Rust
    let len = calculate_length_borrow(&s1);
    println!("The length of '{s1}' is {len}.");

    //borrowing does not take ownership, it is more like borrow it
    //also, we can't modify the value we borrowed
    //We're not allowed to modify something we have a reference to(borrowed)
}

//Note the type is &String which indicates that it is a reference
fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

//moving ownership back and forth is tedious, imagine this:
//we have a string s1, and we want to calculate its length with a function
//then after we calculate its length, we print the String and its length.
fn explain_tedious_moving() {
    //first we define a s1
    let s1 = String::from("hello");
    //then we pass it to function to calculate the length
    let len = calculate_length(s1);
    println!("the lenth of s1 is {len}");
    //now because we move the ownership of s1 to the function
    //we can't use s1 now
    //what should we do now?
    //we can use the tuple as a return value
    //in the tuple, it will contain both the string and its length
    //like this:
    let s2 = String::from("hello");
    //we need to use a tuple, it is tedious
    let (s, len) = calculate(s2);
    println!("the length of '{s}' is {len}");
}

fn calculate(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length(s: String) -> usize {
    return s.len();
}

fn explain_move_calling_function() {
    let s = String::from("hello");
    //s is moved to the function
    take_ownership(s);
    //after that, s is no longer valid here

    let x = 5;
    makes_copy(x);
}

fn take_ownership(s: String) {
    println!("s is {s}");

    //when this function is finished, the drop will be called automatically
    //and the memory of s is freed.
}

fn makes_copy(x: i32) {
    println!("x is {x}");
    //x is also no longer valid, but it is copied so nothing special here
}

fn explain_move() {
    //first we create a string named s1
    //s1 will point to a piece of bytes in the heap
    let s1 = String::from("hello");

    //then we assign s1 to s2
    //a new pointer is pointing to the same bytes in the heap
    //there are two pointers, but the data in the heap is only one!
    let s2 = s1;
    println!("s2={s2}");
    //but after this, we can't use s1 any more
    //because rust treat "s2=s1" as a Move operation, the s1 was moved to s2
    //so s1 is not valid any more
    //actually it is a lille bit like "Shallow copy", but it is not the same.
    // println!("{s1}");

    //if we still want to use s1, we can use a deep clone:
    //let s2 = s1.close();

    //but for value whose size can be known at compile time, it is ok
    //because it is stored in the stack, values are copied,
    //it is copied because it is on stack and it is quick
    let x = 5;
    let y = x;
    //we can still use both x and y
    println!("x={x},y={y}");
}

fn explain_scope() {
    //s is only valid in the block
    {
        //s must be declared as mut explictly,
        //because we will call the push_str method to change it
        let mut s = String::from("hello");

        s.push_str(" world!");
        println!("{s}");
    }
}
