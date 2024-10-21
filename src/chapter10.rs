#[test]
fn test1_1() {
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}
}

#[test]
fn test1_2() {
fn sum<T:std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}
}

#[test]
fn test1_3() {
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
}

#[test]
fn test1_4() {
// modify this struct to make the code work
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // DON'T modify here
    let p = Point{x: 5, y : "hello".to_string()};
}
}

#[test]
fn test1_5() {
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
}

#[test]
fn test1_6() {
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}
}

#[test]
fn test1_7() {
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5.0_f32, y: 10.0_f32};
    println!("{}",p.distance_from_origin())
}
}

#[test]
fn test2_1() {

}

#[test]
fn test2_2() {

}

#[test]
fn test2_3() {

}

#[test]
fn test3_1() {

}

#[test]
fn test3_2() {

}

#[test]
fn test3_3() {

}

#[test]
fn test3_4() {

}

#[test]
fn test3_5() {

}

#[test]
fn test3_6() {

}

#[test]
fn test3_7() {

}

#[test]
fn test3_8() {

}

#[test]
fn test3_9() {

}

#[test]
fn test4_1() {

}

#[test]
fn test4_2() {

}

#[test]
fn test4_3() {

}

#[test]
fn test4_4() {

}

#[test]
fn test4_5() {

}

#[test]
fn test5_1() {

}

#[test]
fn test5_2() {

}

#[test]
fn test5_3() {

}

#[test]
fn test5_4() {

}

#[test]
fn test5_5() {

}
