#![allow(unused)]

use core::num;
use core::prelude::v1;
use std::fmt::Arguments;
use std::{array, io};
use rand::Rng;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn greetngs() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("{}, {}", greeting, name.trim_end());
}

fn constants() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14159;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Please type a number!");
    age += 1;
    print!("Age: {} and I want ${}\n", age, ONE_MIL);
 }

fn data_types() {
    print!("Max u32: {}\n", u32::MAX);
    print!("Max i32: {}\n", i32::MAX);
    print!("Max u64: {}\n", u64::MAX);
    print!("Max i64: {}\n", i64::MAX);
    print!("Max u128: {}\n", u128::MAX);
    print!("Max i128: {}\n", i128::MAX);
    print!("Max f32: {}\n", f32::MAX);
    print!("Max f64: {}\n", f64::MAX);

    let is_true = true;
    let my_grade: char = 'A';
    print!("{} {}\n", is_true, my_grade);
    
    let num_1: f32 = 1.1111111111111;
    print!("f32: {}\n", num_1 + 0.1111111111111);
    let num_2: f64 = 1.1111111111111;
    print!("f64: {}\n", num_2 + 0.1111111111111);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    print!("5 + 4 = {}\n", num_3 + num_4);
    print!("5 - 4 = {}\n", num_3 - num_4);
    print!("5 * 4 = {}\n", num_3 * num_4);
    print!("5 / 4 = {}\n", num_3 / num_4);
    print!("5 % 4 = {}\n", num_3 % num_4);
    num_3 += 1;
    print!("num_3 += 1: {}\n", num_3);
}

fn random_number() {
    let random_num = rand::thread_rng().gen_range(1..101);
    print!("Random: {}\n", random_num);
}

fn if_expression() {
    println!("How old r u?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please type a number!"); // Convert age to u32
    
    if (age >= 1) && (age <= 5) {
        print!("Go to kindergarten\n");
    } else if (age > 5) && (age <= 18) {
        print!("Go to grade school\n");
    } else if (age > 18) && (age <= 21) {
        print!("Go to college\n");
    } else {
        print!("Do what you want\n");
    }
}

fn ternary_operator() {
    println!("How old r u?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please type a number!"); // Convert age to u32

    let can_vote = if age >= 18 {true} else {false};
    print!("Can vote: {}\n", can_vote);
}

fn match_example(){
    println!("How old r u?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please type a number!"); // Convert age to u32

    match age {
        0 => println!("Not born yet"),
        1..=5 => println!("Go to kindergarten"),
        6..=18 => println!("Go to grade school"),
        19..=21 => println!("Go to college"),
        22..=60 => println!("Go get a job"),
        _ => println!("Do what you want"),
    }

    let voting_age =18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Too young to vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("First time voting"),
    }
}

fn arrays(){
    let arr_1 = [1, 2, 3, 4];
    print!("arr_1: {:?}\n", arr_1);
    print!("arr_1[0]: {}\n", arr_1[0]);
    print!("arr_1.len(): {}\n", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5,6,7,8,9];
    let mut loop_index = 0;
    loop {
        if loop_index >= arr_2.len() {
            break;
        }
        print!("arr_2[{}]: {}\n", loop_index, arr_2[loop_index]);
        loop_index += 1;
    }
    print!("\n");

    for i in 0..arr_2.len() {
        print!("arr_2[{}]: {}\n", i, arr_2[i]);
    }

    print!("\n");

    for val in arr_2.iter() {
        print!("val: {}\n", val);
    }

    print!("\n");

    loop_index = 0;
    while loop_index < arr_2.len() {
        print!("arr_2[{}]: {}\n", loop_index, arr_2[loop_index]);
        loop_index += 1;
    }

    print!("\n");
}

fn tuples(){
    let tup_1 = (1, 2, 3, 4);
    print!("tup_1: {:?}\n", tup_1);
    print!("tup_1.0: {}\n", tup_1.0);
    print!("tup_1.1: {}\n", tup_1.1);
    print!("tup_1.2: {}\n", tup_1.2);
    print!("tup_1.3: {}\n", tup_1.3);

    let (x, y, z, a) = tup_1;
    print!("x: {}\n", x);
    print!("y: {}\n", y);
    print!("z: {}\n", z);
    print!("a: {}\n", a);

    let my_tuple = (47, "Derek".to_string(), 50_000.00);
    print!("Name: {}\n", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    print!("Age: {}\n", v1);
}

fn string_example(){
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" string");
    print!("st1: {}\n", st1);
    for word in st1.split_whitespace() {
        print!("{}\n", word);
    }

    let st2 = st1.replace("A", "Another");
    print!("st2: {}\n", st2);

    let st3 = String::from("d 2 d g f k o k ggggg"
);
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        print!("{}\n", char);
    }

    let st4: &str = "Hello there";
    let mut st5 = st4.to_string();
    println!("st5: {}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..5];
    println!("st6.len(): {}", st6.len());
    println!("st6: {}", st6);
    st5.clear();
    
    let st6 = String::from("Hi");
    let st7 = String::from(" there");
    let st8 = st6 + &st7;
    println!("st8: {}", st8);
    for c in st8.chars() {
        println!("{}", c);
    }
    for c in st8.chars().rev() {
        println!("{}", c);
    }
    for c in st8.bytes() {
        println!("{}", c);
    }
}

fn casting(){
    let int_u8: u8 = 5;
    let int2_u8: u8 = 10;
    let int3_u32: u32 = int_u8 as u32 + int2_u8 as u32;
    println!("int3_u32: {}", int3_u32);
}

fn enum_example(){
    enum Day {
        Mon, Tues, Wed, Thurs, Fri, Sat, Sun
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sat | Day::Sun => return true,
                _ => return false,
            }
        }
    }

    // let today:Day = Day::Tues;
    let today:Day = Day::Sat;
    match today {
        Day::Mon => println!("It's Monday"),
        Day::Tues => println!("It's Tuesday"),
        Day::Wed => println!("It's Wednesday"),
        Day::Thurs => println!("It's Thursday"),
        Day::Fri => println!("It's Friday"),
        Day::Sat => println!("It's Saturday"),
        Day::Sun => println!("It's Sunday"),
    }

    println!("Is today a weekend? {}", today.is_weekend());
}

fn vectors_example(){
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    print!("vec2: {:?}\n", vec2);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => print!("2nd: {}\n", second),
        None => print!("No third element\n"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    print!("vec2: {:?}\n", vec2);
    println!("Vec Length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
    for i in &vec2 {
        println!("{}", i);
    }
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    // num1 + num2
    return num1 + num2
}
fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &i in list.iter() {
        sum += &i;
    }
    sum
}
fn function_example(){
    println!("{}", get_sum(5, 4));

    let (val1, val2) = get_2(3);
    println!("Val1: {}, Val2: {}", val1, val2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list: {}", sum_list(&num_list));
}

use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x:T, y:T) -> T {
    x + y
}
fn generic_example(){
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.4 + 4.4 = {}", get_sum_gen(5.4, 4.4));
}

fn print_str(x: String){
    println!("A string {}", x);
}
fn print_return_str(x: String) -> String {
    println!("A string2 {}", x);
    x
}
fn change_string(x: &mut String){
    x.push_str(" is happy");
    println!("Message: {}", x);
}
fn ownership(){
    let str1 = String::from("World");
    // let str2 = str1;
    let str2 = str1.clone();
    println!("Hello {}", str1);

    print_str(str1.clone());
    let str3 = print_return_str(str1);
    println!("str3 = {}", str3);

    let mut str4 = String::from("Derek");
    change_string(&mut str4);
}

use std::collections::HashMap;
fn hashmaps(){
    let mut heros = HashMap::new();
    heros.insert("Superman", "Clark Kent");
    heros.insert("Batman", "Bruce Wayne");
    heros.insert("Spiderman", "Peter Parker");
    heros.insert("The Flash", "Barry Allen");

    for (k, v) in heros.iter() {
        println!("{}: {}", k, v);
    }
    println!("Length: {}", heros.len());
    if heros.contains_key(&"Batman") {
        let the_batman = heros.get(&"Batman");
        println!("the_batman: {}", the_batman.unwrap());
        match the_batman {
            Some(v) => println!("The Batman = {}", v),
            None => println!("No Batman"),
        }
    }
}

fn struct_example(){
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.56,
    };
    println!("{} has {} at {}", bob.name, bob.balance, bob.address);
    bob.address = String::from("123 Elm St");
    println!("{} has {} at {}", bob.name, bob.balance, bob.address);

    struct Rectangle<T, U> {
        width: T,
        height: U,
    }
    let rec = Rectangle {
        width: 10,
        height: 20.5,
    };
    
    trait Shape {
        fn new(width: f32, height: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rect {
        width: f32,
        height: f32,
    }
    struct Circle {
        radius: f32,
    }
    impl Shape for Rect {
        fn new(width: f32, height: f32) -> Rect {
            Rect {
                width: width,
                height: height,
            }
        }
        fn area(&self) -> f32 {
            self.width * self.height
        }
    }
    impl Shape for Circle {
        fn new(width: f32, height: f32) -> Circle {
            Circle {
                radius: width,
            }
        }
        fn area(&self) -> f32 {
            3.14159 * (self.radius * self.radius)
        }
    }

    let rec: Rect = Shape::new(10.0, 20.0);
    let circ: Circle = Shape::new(1.0, 20.0);

    println!("Area of Rect: {}", rec.area());
    println!("Area of Circle: {}", circ.area());
}

mod restaurant;
use crate::restaurant::order_food;

fn modules_examle(){
    order_food();
}

fn main() {
    // greetngs();
    // constants();
    // data_types();
    // random_number();
    // if_expression();
    // ternary_operator();
    // match_example();
    // arrays();
    // tuples();
    // string_example();
    // casting();
    // enum_example();
    // vectors_example();
    // function_example();
    // generic_example();
    // ownership();
    // hashmaps();
    // struct_example();
    modules_examle();
}

