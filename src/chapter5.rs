/// https://practice.course.rs/ownership/ownership.html

#[test]
fn test11() {
 fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
}

#[test]
fn test12() {
  // Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
}

#[test]
fn test13() {
  fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
}
}

#[test]
fn test14() {
 fn main() {
     let s = String::from("hello, world");
     print_str(&s);
     println!("{}", s);
 }
 fn print_str(s: &String)  {
     println!("{}",s)
 }}

#[test]
fn test15() {
  fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
}

#[test]
fn test16() {
 fn main() {
    let s = String::from("hello, ");
    
    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")
}
}

#[test]
fn test17() {
 fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);       // implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);
}
}

#[test]
fn test18() {
fn main() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // modify this line only, don't use `_s`
    println!("{:?}", t.1);
 }}

#[test]
fn test19() {
  fn main() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
}

#[test]
fn test21() {
    // Код теста для 2.1
}

#[test]
fn test22() {
    // Код теста для 2.2
}

#[test]
fn test23() {
    // Код теста для 2.3
}

#[test]
fn test24() {
    // Код теста для 2.4
}

#[test]
fn test25() {
    // Код теста для 2.5
}

#[test]
fn test26() {
    // Код теста для 2.6
}

#[test]
fn test27() {
    // Код теста для 2.7
}

#[test]
fn test28() {
    // Код теста для 2.8
}

#[test]
fn test29() {
    // Код теста для 2.9
}

#[test]
fn test210() {
    // Код теста для 2.10
}

#[test]
fn test211() {
    // Код теста для 2.11
}
