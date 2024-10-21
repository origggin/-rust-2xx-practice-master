//Yak zhe ya zadovbavsya
#[test]
fn test1_1() {
///cargo new hello-package
}

#[test]
fn test1_2() {
///cargo new --lib hello-package1
}

#[test]
fn test1_3() {

}

#[test]
fn test1_4() {

}

#[test]
fn test1_5() {
# FILL in the blanks
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── lib.rs
}

#[test]
fn test1_6() {
# Create a package which contains 
# 1. three binary crates: `hello-package`, `main1` and `main2`
# 2. one library crate
# describe the directory tree below
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests # directory for integrated tests files
│   └── some_integration_tests.rs
├── benches # dir for benchmark files
│   └── simple_bench.rs
└── examples # dir for example files
    └── simple_example.rs
}

#[test]
fn test2_1() {
// in lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        fn complain() {} 
    }
}
}

#[test]
fn test2_2() {
// in lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}

        // Maybe you don't want the guest hearing the your complaining about them
        // So just make it private
        fn complain() {} 
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
}

#[test]
fn test2_3() {
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
}

#[test]
fn test2_4() {
// in src/lib.rs

mod front_of_house;
mod back_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
}

#[test]
fn test2_5() {
mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
}

#[test]
fn test3_1() {
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {}
}

#[test]
fn test3_2() {
use std::collections::*;

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}
}

#[test]
fn test3_3() {
// in lib.rs

// Add this line
pub use crate::front_of_house::hosting;
}
