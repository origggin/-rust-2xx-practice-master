#[test]
fn test11() {
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}
}

#[test]
fn test12(){
fn main() {
    let boolean = true;

    // fill the blank with an match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);
}
}

#[test]
fn test13(){
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move{x: a, y: b} => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}
}
#[test]
fn test14(){
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
} 
}

#[test]
fn test15(){
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e , MyEnum::Foo) { // fix the error with changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);
}
}

#[test]
fn test16(){
fn main() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}
}

#[test]
fn test17(){
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
    }
}
}

#[test]
fn test18(){
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ =>  println!("match others")
    }
}
}

#[test]
fn test19(){
fn main() {
    let age = Some(30);
    if let Some(age) = age { // create a new variable with the same name as previous `age`
       assert_eq!(age, 30);
    } // the new variable `age` goes out of scope here
    
    match age {
        // match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }
}

#[test]
fn test21(){
fn main() {}
fn match_number(n: i32) {
    match n {
        // match a single value
        1 => println!("One!"),
        // fill in the blank with `|`, DON'T use `..` ofr `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}
}

#[test]
fn test22(){

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // fill in the blank to let p match the second arm
    let p = Point { x: 2, y: 20 }; // x can be [0, 5], y can be 10 20 or 30

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
}

#[test]
fn test23(){
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  id@3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@(10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
}

#[test]
fn test24(){
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}
}

#[test]
fn test25(){
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }
}
}

#[test]
fn test26(){
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        // The type of value is &mut String
       value => value.push_str(" world!") 
    }
}
}
