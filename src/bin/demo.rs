use std::thread::{self, JoinHandle};
use std::time::Duration;

fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let add = Box::new(|a, b| {
        println!("adding!");
        a + b
    });
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, mul));

    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        data
    });

    println!("Waiting for value ...");
    match caps.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error: {:?}", e),
    }

    let value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("Waiting on thread");

    match value.join() {
        Ok(n) => println!("value: {}", n),
        Err(e) => println!("error: {:?}", e),
    }
}