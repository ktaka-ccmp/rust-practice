// https://youtu.be/TJTDTyNdJdY

/// Represents a `SomeStruct` with a single `num` field.
#[derive(Debug)]
struct SomeStruct {
    num: i32,
}

/// Prints the debug representation of a `SomeStruct`.
fn print_some_struct(s: &SomeStruct) {
    println!("SomeStruct: {:?}", s);
}

/// Returns a reference to the biggest `SomeStruct` between `s1` and `s2`.
/// The lifetime `'lf` is used to ensure that the returned reference is valid as long as both `s1` and `s2` are valid.
fn biggest<'lf>(s1: &'lf SomeStruct, s2: &'lf SomeStruct) -> &'lf SomeStruct {
    if s1.num > s2.num {
        s1
    } else {
        s2
    }
}

/// Demonstrates lifetime usage with `SomeStruct`.
pub(crate) fn lt1() {
    let mut s = SomeStruct { num: 42 };
    print_some_struct(&s);

    let bigger: &SomeStruct;
    // {
        let s2 = SomeStruct { num: 100 };
        bigger = biggest(&s, &s2);
    // }
    print_some_struct(bigger);
}

/// Represents an `OtherStruct` with a single `num` field that holds a reference to an `i32`.
#[derive(Debug)]
struct OtherStruct<'lf> {
    num: &'lf i32,
}

/// Prints the debug representation of an `OtherStruct`.
fn print_other_struct(s: &OtherStruct) {
    println!("OtherStruct: {:?}", s);
}

/// Demonstrates lifetime usage with `OtherStruct`.
pub(crate) fn lt2() {
    let s: OtherStruct;
    // {
        let x = 4;
        s = OtherStruct { num: &x };
    // }
    print_other_struct(&s);
}

