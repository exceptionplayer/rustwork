use rand::Rng;
use std::{thread, time};

fn main() {
    let m = max(1, 3);
    println!("{m}");

    let n = if m == 1 { true } else { false };
    println!("{n}");

    loop_test();
    multiple_if(rand::thread_rng().gen_range(-100..100));

    let got = return_value_from_loop(0);
    println!("got:{got}");

    loop_with_while();
    loop_collection_with_for();
}

fn loop_collection_with_for(){
    let a = [1,2,3,4,5];
    for value in a {
        println!("value is {value}");
    }
}

fn loop_with_while(){
    let mut count = 5;
    while count>0{
        println!("count:{count}");
        count-=1;
    }
}

fn return_value_from_loop(expected: i32) -> bool {
    let mut rng = rand::thread_rng();
    //let the random to guess 5 times at most
    //return true if succeed false otherwise
    let mut remain_chance = 5;
    loop {
        let rand_value = rng.gen_range(-100..100);
        remain_chance -= 1;

        if rand_value == expected {
            //break and return has the same effect here
            // the last expression is the return value of this function
            // return true;
            break true;
        }

        if remain_chance == 0 {
            // return false;
            break false;
        }
    }
}

fn loop_test() {
    let mut count = 0;

    let one_sec = time::Duration::from_secs(1);
    let mut now = time::Instant::now();

    loop {
        count = count + 1;
        println!("looping, count: {count}, time: {:?}", now);

        thread::sleep(one_sec);
        now = time::Instant::now();

        if count == 10 {
            break;
        }
    }
    println!("loop finished, total count: {count}");
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn multiple_if(x: i32) {
    if x < 0 {
        println!("less than 0");
    } else if x == 0 {
        println!("zeron");
    } else {
        println!("greater than 0");
    }
}
