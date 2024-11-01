#[test]
fn test1_1() {
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
}

#[test]
fn test1_2() {
// FILL in the blanks
fn main() {  
   // get a slice of String with reference: String -> &str 
   let mut s = String::from("hello, world");

   let slice1: &str = &s; // in two ways
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[0..5];
   assert_eq!(slice2, "hello");

   //Note! The type here cant be `&mut str` due to `push` is ONLY defined on String type and its mut reference: `&mut String` !
   // So you can't use `s.as_mut_str()`
   let slice3: &mut String = &mut s; 
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!")
}
}

#[test]
fn test1_3() {
///two heap allocations.
}

#[test]
fn test1_4() {
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];//modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}
}

#[test]
fn test1_5() {
// FILL in the blanks
fn main() {
    let mut s = String::new();
    s.push_str("hello");

    // some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a bytes vector into a String
    // We know these bytes are valid, so we'll use `unwrap()`.
    let s1 = String::from_utf8(v).unwrap();
    
    
    assert_eq!(s, s1);

    println!("Success!")
}
}

#[test]
fn test1_6() {
fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}
}

#[test]
fn test1_7() {
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    // story has nineteen bytes
    assert_eq!(16, len);

    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}
}

#[test]
fn test2_1() {
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);
    
    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE `for` to rewrite the below code 
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);
 
    assert_eq!(format!("{:?}",v), format!("{:?}",v1));

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {}


//Another solution 


fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);
    
    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code 
    let mut v1 = vec!();
    for i in &v{
        v1.push(*i);
    }
    is_vec(&v1);
 
    assert_eq!(v, v1);

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {}

}

#[test]
fn test2_2() {
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend([1, 2, 3]);

    assert_eq!(format!("{:?}",v1), format!("{:?}",v2));

    println!("Success!")
}
}

#[test]
fn test2_3() {
fn main() {
    // array -> Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();
 
    assert_eq!(v1, v2);
 
    
    // String -> Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    println!("Success!")
 }
}

#[test]
fn test2_4() {
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
        if let Some(x) = v.get(i) {
            v[i] = x + 1
        } else {
            v.push(i + 2)
        }
    }
    
    assert_eq!(format!("{:?}",v), format!("{:?}", vec![2, 3, 4, 5, 6]));

    println!("Success!")
}
}

#[test]
fn test2_5() {
// FIX the errors
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[0..3];
    
    assert_eq!(slice1, slice2);
    
    // A slice can also be mutable, in which
    // case mutating it will also mutate its underlying Vec
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..4];
    slice3[3] = 42;

    assert_eq!(slice3, &[1, 2, 3, 42]);
    assert_eq!(v, &[1, 2, 3, 42]);

    println!("Success!");
}
}

#[test]
fn test2_6() {
// FIX the errors
fn main() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


    // fill in an appropriate value to make the `for` done without reallocating 
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!")
}
}

#[test]
fn test2_7() {
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    // FILL in the blank
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}
}

#[test]
fn test2_8() {
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
}

#[test]
fn test3_1() {
// FILL in the blanks and FIX the erros
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // get returns a Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // indexing return a value V
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}
}

#[test]
fn test3_2() {
use std::{collections::HashMap};
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2 = HashMap::from(teams);
    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}
}

#[test]
fn test3_3() {
// FILL in the blanks
use std::collections::HashMap;

fn main() {
    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    // could actually return some random value here - let's just return
    // some fixed value for now
    42
}
}

#[test]
fn test3_4() {
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
}

#[test]
fn test3_5() {
use std::collections::HashMap;

fn main() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    // &str implements Copy trait
    let v2 = "hello";
    let mut m2 = HashMap::new();
    m2.insert(v2, v1);

    assert_eq!(v2, "hello");
}
}
