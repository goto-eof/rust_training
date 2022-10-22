macro_rules! add {
    ($a:expr, $b:expr, $c:expr) => {
        $a + $b + $c
    };
    ($a:expr, $b: expr ) => {
        $a + $b
    };

    ($a:expr, $b:expr, $c: expr, $type: ty) => {
        $a as $type + $b as $type + $c as $type
    };
}

macro_rules! add_alternative {
    ($a:expr) => {
        $a
    };

    ($a: expr, $($b:tt)*) => {
        $a + add_alternative!($($b)*)
    };
}

macro_rules! non_fixed_num_arguments {

    ($($a:expr), *) => {
        0
        $(+$a)*
    };
}

macro_rules! single_element {
    ($a:expr) => {
        $a
    };
}

macro_rules! validate {
    ($($opt:expr), *) => {{
        $(
            match $opt>= 0 && $opt<=9{
                true => {println!( "Valid: {:?}",    $opt);},
                false => {println!( "Invalid: {:?}",    $opt);},
            }
        )*
    }};
}

macro_rules! preprocessing {
    ($fun:ident, $($values:expr), *) => {
        fn preprocess (i:i32)-> i32{
            i * 5
        }
        $fun($(preprocess($values)), *);
    };
}

fn test(a: i32, b: i32) {
    println!("a: {}, b: {}", a, b);
}

macro_rules! if_ok_then_execute_fn {
    ($fun: ident, $($value: expr), *) => {
       {$(
        match $value {
            1 => $fun($value),
            num => println!("is not ok: {}", num)
        }
    )*
       }
    };
}

fn fn_if_ok_then_execute_fn(i: i32) {
    println!("is ok: {}", i);
}

fn main() {
    println!("{}", add!(1, 2));
    println!("{}", add!(1, 2, 3));
    println!("{}", add!(1, 2, 3, u16));
    println!("{}", single_element!(1));
    println!("{}", non_fixed_num_arguments!(1, 2, 3));
    println!("{}", add_alternative!(1, 2, 3, 4, 5, 6, 7, 8, 9));
    validate!(1, 2, 10, 3);
    preprocessing!(test, 3, 4);

    if_ok_then_execute_fn!(fn_if_ok_then_execute_fn, 1, 2, 3, 4, 5, 6);
}
