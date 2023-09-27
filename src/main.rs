#![allow(unused_imports)]

use std::collections::HashMap;
use serde_json::{json, Value};

fn main() {
    println!("hello world!");

    // Replacing dictionaries with hashmaps

    #[derive(Debug)]
    enum Value {
        Str(&'static str),
        Int(isize),
    }

    let mut map = HashMap::new();

    map.insert("one", Value::Str("1"));
    map.insert("two", Value::Int(2));

    for (_, v) in &map {
        match v {
            Value::Str(a) => {
                println!("{}", *a);
            },
            Value::Int(a) => {
                println!("{}", a);
            }
        }
    }

    let outcome = map.get("test");
    println!("{:?}", outcome); // --> None

    // let outcome = map.get("test").unwrap();
    // println!("{:?}", outcome); // --> thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', main.rs:33:35

    match map.get("test") {
        Some(a) => {
            println!("{:?}", a);
        },
        None => {
            println!("None returned");
        },
    } // --> None returned

    fn process_enum(value: &Value) {
        match value {
            Value::Str(a) => {
                println!("str :: {}", a);
            },
            Value::Int(a) => {
                println!("int :: {}", a);
            },
        }
    }

    match map.get("test") {
        Some(value) => {
            process_enum(value);
        },
        None => {
            println!("There is no value");
        },
    } // --> There is no value

    fn add_numbers(one: isize, two: isize) -> isize {
        one + two
    }

    let result = add_numbers(10, 20);
    println!("{}", result);

    let stock = json!({
        "name": "두산에너빌리티",
        "price": 70800,
        "history": [16400, 21300, 45000, 68000],
    });

    println!("last price is : {}", stock["history"][3]);
    println!("{}", stock.to_string());

}