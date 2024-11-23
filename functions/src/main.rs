use rand::Rng;

fn main() {
    show_values(1, false);
    statement_and_expression();

    let x = add(1, 2);
    println!("{x}");
    println!("{}", max(2, 1));
    println!("{}", max(1, 2));

    let random_value = rand::thread_rng().gen_range(0..=100);

    //we can use if in one line
    //but the if and else block must retutn the same type
    let y = if random_value > 50 {true} else {false};
    println!{"{y}"};
}




//if we need a return value, we must define the type of the value using `->`
fn add(x: i32, y: i32) -> i32 {
    //the final expression of the function body is the return value
    x + y
}

//we can also return early with the keyword `return`
fn max(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    } else {
        return y;
    }
}

fn statement_and_expression() {
    // let x = 10; //this is a statement
    // 1 + 2; --> this is an expression

    //we can't do this, because statement does not return any value
    // let x = (let y = 6);

    // A new scope block created with curly brackets is an expression
    let y = {
        let x = 3;
        // Note: there is no semicolon in the end of the following line
        x + 1
    };
    //the result of y is 4, the "x + 1" in the block is 4
    //and there is no semicolon at the end, so it is an expression,
    //an expression will return a value, so the value 4 is returned
    println!("The value of y is: {y}");
}

fn show_values(value1: i32, value2: bool) {
    println!("value1:{value1}, value2:{value2}");
}
