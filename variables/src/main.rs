fn main() {
    variables();
    constants();
    shadowing();
    scalar_types();
    tuple_types();
    array_types();
}

fn array_types(){
    //define an array without specifiying the element type and array size
    let a = [1,2,3,4,5];
    println!("the 2nd element of the array is: {}", a[1]);

    //define an array with the element type and array size specified
    //please note: it use a semicolon.
    let a: [i32; 5] = [1,2,3,4,5];
    println!("the 3rd element of the array is: {}", a[2]);

    //define an array with same value for n size
    //please not the semicolon
    let a = [3;5];
    println!("the fifth element of the array is: {}", a[5]);
}

fn tuple_types(){
    //define a tuple without specifying the type of each element
    let tup = (100,10,1);
    //this is called "destructuring"
    let (x,y,z) = tup;
    println!("x is: {x}, y is: {y}, z is: {z}");


    //define a tuple with type of each element is specified
    let tup: (i32, f64, bool) = (10, 0.3, true);
    //use dot and index to visit the elements
    println!("values are: {}, {}, {}", tup.0, tup.1, tup.2);
}

fn scalar_types(){
    //integers
    let x  = 32; //it will  be inferred as i32
    println!("{x}");
    let x: i32 = 100;
    println!("{x}");
    //integers have different literals.

    //decimal
    let x = 10_000;
    println!("{x}");
    //hex
    let x = 0xff;
    println!("{x}");
    //octal
    let x = 0o77;
    println!("{x}");
    //binary
    let x = 0b111_00;
    println!("{x}");
    //byte(u8) only
    let x = b'A';
    println!("{x}");

    //floating-point types

    //f64 and f32
    let x = 2.0; //f64, it is the default type
    println!("{x}");
    let y: f32 = 3.0; //f32
    println!("{y}");

    //boolean
    let x = true; 
    println!("{x}");
    let x:bool = false;
    println!("{x}");
}

fn shadowing() {
    //Shadowing
    //Shadowing is different from marking a variable as mut 
    //because we’ll get a compile-time error 
    //if we accidentally try to reassign to this variable 
    //without using the let keyword. 
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        //the value will be 12
        println!("The value of x in the inner scope is: {x}");
    }

    //the value will be 6,
    println!("The value of x is: {x}");

    //Also we can change the type of x by using Shadowing
    let x = "Hello";
    println!("The value of x is: {x}");

    //Also we can chagne the mutability of a variable by using Shadowing
    let mut x = "hala";
    println!("The value of x is: {x}");
    x = "haaa";
    println!("The value of x is: {x}");
}


fn constants() {
    //Constants
    const TIMEOUT_SEC: u32 = 10;
    println!("The constant value of timeout is {TIMEOUT_SEC}");
}

fn variables() {
    //Variables
    
    //variables are immutable by default, so if we define x like this
    //`let x = 5`; there will be compiler error.
    //To make it mutable, we can use the mut keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
