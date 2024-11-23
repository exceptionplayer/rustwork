1. calling with a `!` means you are calling a marco instead of a function, like `println("hello")` , the `println` is a macro instead of a function.
2. we can use `rustfmt` to format our code.
3. cargo build 
``` 
We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
```
4. cargo release
```
When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug.
```

5. rust provides some defined standard libraries, they can be found here: https://doc.rust-lang.org/std/prelude/index.html

6. we can use `use` keyword to "import" something like java.

- we use `let` to create a variable, **variables are immutable** by default in Rust.
- we use `let mut` to create a mutable variable
- the & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use. For example, the `rand` is a crate.
- `cargo update` can be used to upgrade all the dependencies in the project.
- `Cargo.lock` file is generated automatically, it is used to make sure your build is stable and reproducable, it will contain the dependencies the project depends on and if you don't update the dependencies, it will not change so that some version chooing work can be omitted when cargo build is running.
- we can use `cargo doc --open` to open the documentation based on the project dependencies.
- Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, for example:
    ```
    let mut guess = String::new();
    //we can reuse the variable name "guess", this is called Shadowing
    let guess: i32 = guess.trim().parse().expect("Please input a number");
    ``` 

# Code and Tools
- Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

# Variables and Mutability
1. By default, variables are immutable in Rust, but we can make them mutable by adding `mut`.
2. Constants are values that are bound to a name and are not allowed to change.
  - can't use `mut` with a contant.
  - you declare a constant with `const` keyword instead of the `let` keyword.
  - you must specify the type of the constant.
  - Rust’s naming convention for constants is to use all uppercase with underscores between words.
3. Shadowing is different from `mut` in several ways:
  - We must use `let` keyword, if we forget `let`, there will be complier errors.
  - After shadowing, the varible is still immutable.
  - We can also change the type of the variable by using Shadowing.
  - We can also change the mutability of the variable by using Shadowing.(tested by myself, but haven't seen any documents yet)
4. Types:
  - Rust is a statically typed language, which means it must know the types of all variables at compile time.
  - Scalar types: integers, floating-point numbers, booleans and characters.
    - Integer types: i8,u8..., there are `isize` and `usize` types which mean the size of a pointer, the length of the two depends on the arch of you computer, if it is 64-bit architecture, then it is 64bits.
  - Compound types: 1) Tuple 2) Array
    - Tuple: A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
      - The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
    - Array: every element is an array should have the same type.

# Functions
- Define functions with keyword: "fn".
- In function signatures, you must declare the type of each parameter.
- Q: How can I define a function with an array or tuple type parameter?
- Statement and Expressions:
  - Statements are instructions that perform some action and do not return a value.
  - Expressions evaluate to a resultant value. 
  - Because of this, we can't write code like `x = y = 6` in other languages.
  - A new scope block created with curly brackets is an expression.
  - **Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.**
  - if we need a return value, we must define the type of the value using `->`.
  - the final expression of the function body is the return value, if we want to return early, we can use the `return` keyword.

# Ownership
- Brief intruction:
  - Adding data is called pushing onto the stack, and removing data is called popping off the stack. 
  - All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
  - Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
  - Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
  - Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
  - Once you understand ownership, you won’t need to think about the stack and the heap very often, but **knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does**.

- Ownership rules
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.

- In Rust, the memory is automatically returned once the variable that owns it goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. **Rust calls `drop` automatically at the closing curly bracket**.

- **Move** operation for variables that are allocated on heap:
  ``` 
    let s1 = String::from("hello");
    let s2 = s1; //s1 can't be used any more because it is moved to s2;
  ```
  - If the value is on stack, it is copied instead of moved:
  ``` let x = 1;
       let y = x; //both x and y can still be used because it is copied.
  ```
  - The `Copy` trait indicates that the types are stored on the stack, for those types, they are not moved but copied.
  - The `Drop` trait indicates that the types are dropped when they are out of scope, these types are moved.

- Passing a value to a function are similar to those when assigning a value to a variable. It can be a copy or a move operation.
- Returning values can also transfer ownership.

- But moving ownership back and forth is tedious, so Rust introduced references.

- Reference: it allow you to refer to some value **without taking ownership** of it.
It is defined with the `&`.
-  When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.
- Just as variables are immutable by default, so are references.
- We call the action of creating a reference borrowing. 
- We’re not allowed to modify something we have a reference to.
- We can use mutbale references if we want to modify the value.
- Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

- Slice:
  - A slice is a kind of reference, so it does not have ownership.