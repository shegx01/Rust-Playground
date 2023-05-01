#![allow(unused)]
use core::mem::*;
use std::{
    collections::{self, HashMap},
    f32::consts::PI,
    fs,
    io::{self, BufRead},
    process, vec, ops::Mul,
};

fn main() {
    #![allow(unused)]
    #![cfg(unix)] // configuration for enclosing items using !-> for unix
    use std::mem::*;
    let o = 0;
    let mut map: HashMap<String, &str> = HashMap::new();
    o.mul(4);
    // Iterator
    let u = "नमस्ते";
    let u2: String = u.chars().take(1).collect();

    let lst = &u[0..].bytes().collect::<Vec<u8>>();
    println!("lst is {:?}", lst);
    let str = String::from("Hello world");

    let val = Some(20);
    let non_val: Option<i32> = None;
    assert_eq!(val.unwrap(), 20);
    // let vec = Vec::<i32>::new();

    let res: Result<String, &str> = Ok(String::from("Hello"));
    let res_error: Result<_, &str> = Err::<&str, &str>("Something went wrong");

    //     // Double v1
    //     let doubled = list.iter().map(|&val| val * 2).collect::<Vec<i32>>();

    //     println!("The result is {:?}", doubled);

    //     // Double v2
    //     let doubled: Vec<i32> = list.iter().map(|&val| val.pow(val as u32)).collect();
    //     println!("The result is {:?}", doubled);

    //     let file_path = "./Cargo.lock".to_string();

    //    let _data = match fs::read_to_string(&file_path) {
    //         Ok(data) => {
    //             println!("The file name {:?}", data);
    //             data
    //         }
    //         Err(_err) => process::exit(2),
    //     };

    // let odd = list
    //     .iter()
    //     .filter(|&val| val % 2 != 0).collect::<Vec<_>>();
    //             println!("The result is  {:?}", odd);

    // let result = list
    //     .iter()
    //     .fold(0, |acc, x| acc + x);
    //             println!("The result is  {:?}", result);

    #[derive(Debug, Clone)]
    struct Pairs<T: Sized> {
        x: T,
        y: T,
    }

    let character = '*';
    // Both identical
    let mut user = String::from("user");
    let mut user = "user".to_string();

    let pair: Pairs<i32> = Pairs { x: 3, y: 2 };

    let cloned_pair: Pairs<_> = pair.clone();

    println!("The value is: {:?}", pair);
    println!("The value is: {:?}", cloned_pair);

    struct KV(String, u64);

    let kv = KV(String::from("James"), 32);

    #[derive(Debug)]
    enum AgeGroup {
        Young,
        Old,
        CreeptyOld(u32),
    }

    AgeGroup::CreeptyOld(120);

    let val = Box::<AgeGroup>::new(AgeGroup::Young);

    let mut owner = String::from("you");

    owner = String::from("Them");

    owner.push('.');

    let owner2 = "Them".to_string();

    let owner_slice = &owner2[2..];

    println!("The owner slice is {}", owner_slice);

    // only if in range 0 to 255
    let val = char::from(65);
    // convert to unicode codepoint
    let val = char::from_u32(128145);
    let val = char::is_alphabetic('*');
    // get a u32 from a unicode representation
    let val = u32::from('β');
    let val = 'β'.len_utf8();
    let val = u32::from('♤');
    // pow abs ...
    u16::pow(2, 8);
    i32::abs(-6);

    let mut mem1 = "hello";
    let mut mem2 = "world";

    swap(&mut mem1, &mut mem2);

    println!("The swapped item is {}", format!("{}{}", mem1, mem2));

    let data = ("Nigeria", 1998);

    println!("The value of val is {:?}", val);
    println!("The value of val is {}", size_of::<isize>());

    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;

    fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
        let mut numbers = vec![];
        for line_result in file.lines() {
            let line = line_result?;
            numbers.push(line.parse()?);
        }
        Ok(numbers)
    }
    let y = 200;
    let z: i32;

    z = y << 2;

    let res = "Hello Worlds".chars().collect::<Vec<_>>();

    println!("After Shifting,,, {}", z);

    let x = String::from("Hello");
    use std::cell;
    let y = &x;
    let z = &y;
    debug_assert_eq!(x, **z);
    println!("Value of y {}", y);
    println!("Value of z {z}");
    println!("Value of **z {}", **z);

    fn time_sort_run() -> Option<i32> {
        Some(20)
    }

    if let Some(digit) = time_sort_run() {
        debug_assert_eq!(digit, 20);
    };

    println!("u2 is {u2}");
    trait Vegetable {
     
  };
  

    struct Salad {
        veggies: Vec<Box<dyn Vegetable>>
  };
}

// pass the --test-threads to the test executable
// cargo test -- --test-threads

// capture passed tests too
// test -- --no-capture.

// open html doc of your project
// cargo doc --no-deps --open

// No run in comment
// ```no_run
// ```

// totally ignore and not comiling them
// ```ignore
//```

// Specifying Deps
//  image = { git = "https://github.com/Piston/image.git", rev = "528f19c" }
// Specifying a dir
//  image = { path = "vendor/image" }

//  dependency = "=1.0.1" use exact version
//  dependency = ">=1.0.1" user 1.0.1 or highter
//  dependency = ">1.0.1 < 1.1.9" higher than 1.0.1 and less than 1.1.9
//  dependency = "<=2.3.2" exact version or less
