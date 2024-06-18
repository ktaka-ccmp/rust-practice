#![allow(unused)]

mod ex;
mod restaurant;
mod polymorph;
mod lifetime;

use ex::{
    arrays, casting, closure_example, concurrency_example, constants, data_types, enum_example,
    error_handling, function_example, generic_example, greetings, hashmaps, if_expression,
    iterator_example, match_example, modules_example, mutex_example, ownership, random_number,
    smartpointer_example, smartpointer_rct_example, string_example, struct_example,
    ternary_operator, tuples, vectors_example,
};

fn main() {
    // greetings();
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
    // generic_example(); // Difficult to understand
    // ownership();
    // hashmaps();
    // struct_example();
    // modules_example();
    // error_handling(); // Difficult to understand
    // iterator_example();
    // closure_example(); // Difficult to understand
    // smartpointer_example();
    // concurrency_example();
    // smartpointer_rct_example();
    // mutex_example();
    // ex::mutex_example2();
    // polymorph::pm1();
    lifetime::lt2();
}
