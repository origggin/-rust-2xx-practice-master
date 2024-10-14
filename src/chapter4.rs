/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test11() {
        let x: i32 = 5;
        let mut y: i32 = 5;

        y = x;

        let z = 10; // Type of z ?

        println!("Success!");
}

#[test]
fn test12() {
        let v: u16 = 38_u8 as u16;
        println!("Success!");
}

#[test]
fn test13() {
        fn main() {
        let x = 5;
        assert_eq!("i32", type_of(&x)); 

        println!("Success!");
}


fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

}

#[test]
fn test14() {
fn main() {
    assert_eq!(i8::MAX, 127);  
    assert_eq!(u8::MAX, 255);  

    println!("Success!");
}

}

#[test]
fn test15() {
fn main() {
    let v1 = 251_u8.checked_add(8).unwrap(); 
    let v2 = i8::checked_add(251, 8).unwrap(); 

    println!("{}, {}", v1, v2);
}

}

#[test]
fn test16() {
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; 
    assert!(v == 1579); 

    println!("Success!");
}

}

#[test]
fn test17() {
fn main() {
    let x = 1_000.000_1; 
    let y: f32 = 0.12; 
    let z = 0.01_f64; 

    assert_eq!(type_of(&x), "f64".to_string()); 
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

}

#[test]
fn test18() {
fn main() {
    let epsilon = 1e-10; 
    assert!((0.1 + 0.2 - 0.3).abs() < epsilon); 

    println!("Success!");
}

}
///Range
#[test]
fn test19() {
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

}

#[test]
fn test110() {
use std::ops::{Range, RangeInclusive};

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 }); 
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); 

    println!("Success!");
}

}

#[test]
fn test111() {
    fn main() {
    assert!(1u32 + 2 == 3); 
    assert!(1i32 - 2 == -1); 
    assert!(1u8.wrapping_sub(2) == 255);
    assert!(3 * 50 == 150); 
    assert!(9.6 / 3.2 == 3.0); 
    assert!(24 % 5 == 4); 
    assert!(true && false == false); 
    assert!(true || false == true); 
    assert!(!true == false);   
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

}

#[test]
fn test21() {
    // Пример 1 из 4.2
}

#[test]
fn test22() {
    // Пример 2 из 4.2
}

#[test]
fn test23() {
    // Пример 3 из 4.2
}

#[test]
fn test24() {
    // Пример 4 из 4.2
}

#[test]
fn test25() {
    // Пример 5 из 4.2
}

#[test]
fn test26() {
    // Пример 6 из 4.2
}

#[test]
fn test31() {
    // Пример 1 из 4.3
}

#[test]
fn test32() {
    // Пример 2 из 4.3
}

#[test]
fn test33() {
    // Пример 3 из 4.3
}

#[test]
fn test41() {
    // Пример 1 из 4.4
}

#[test]
fn test42() {
    // Пример 2 из 4.4
}

#[test]
fn test43() {
    // Пример 3 из 4.4
}

#[test]
fn test44() {
    // Пример 4 из 4.4
}

#[test]
fn test45() {
    // Пример 5 из 4.4
}
