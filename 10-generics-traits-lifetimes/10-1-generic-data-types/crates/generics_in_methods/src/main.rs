struct Point<T> {
    x: T,
    y: T,
}

// Implement methods on structs & enums, and use generic types in their definitions.
// `x` is a method implemented on the `Point<T>` struct.
// The `x` method returns a reference to the data in field `x`.
//
// (A) - unconstrained generic type `T`
// - methods defined within such an `impl` will be defined on __any__ instance of the type,
//   regardless of what concrete type ends up substituting for the generic type. In contrast, see (B).
// For the `<T>` declaration, any other letter could have been used, eg `<U>`, etc.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//
// (B) - define methods on the `Point<T>` type with some constraint on the generic type,
//   eg by implementing methods only on `Point<f32>` instances.
//
// Measure how far the point is from the point at coordinates(0.0, 0.0):
// - uses a mathematical operation, `powi`, that is available only to floating
//   point types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// (C) Example of generics on an `impl` vs those on a method (the `fn` inside an `impl`)
#[derive(Debug)]
struct Pt<X1, Y1> {
    x: X1,
    y: Y1,
}
// Need to use the `Clone` trait & `clone` method:
// - rust - Cannot move out of borrowed content / cannot move out of behind a shared reference - Stack Overflow
// - https://stackoverflow.com/questions/28158738/cannot-move-out-of-borrowed-content-cannot-move-out-of-behind-a-shared-referen/28159407#28159407
// impl<X1: Clone, Y1: Clone> Pt<X1, Y1> {
//     fn into_mixup<X2: Clone, Y2: Clone>(&self, other: &Pt<X2, Y2>) -> Pt<X1, Y2> {
//         Pt {
//             x: self.x.clone(),
//             y: other.y.clone(),
//         }
//     }
// }
impl<X1, Y1> Pt<X1, Y1> {
    fn into_mixup<X2, Y2>(self, other: Pt<X2, Y2>) -> Pt<X1, Y2> {
        Pt {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // (A) works with `i32` values
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (A) works with `f64` values
    let p = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (A) works with `u8` values
    let p = Point { x: 5u8, y: 10u8 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (A) works with `u16` values
    let p = Point {
        x: 5_u16,
        y: 10_u16,
    };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (B)
    let p = Point { x: 3_f32, y: 4_f32 };
    println!("distance_from_origin: {}", p.distance_from_origin());
    // distance_from_origin: 5

    // (C)
    let p1 = Pt { x: 9, y: 10 };
    let p2 = Pt { x: "Hello", y: 'c' };

    // (C-1) passing references to `mixup`
    // let p3 = p1.into_mixup(&p2);
    // println!("{:?} mixed with {:?} becomes: {:?}", p1, p2, p3);

    // (C-2) passing ownership to `mixup`
    let mut debug_string = format!("{:?} mixed up with {:?} becomes: ", p1, p2);
    let p3 = p1.into_mixup(p2);
    debug_string.push_str(&format!("{:?}", p3));
    println!("{}", debug_string);
}
