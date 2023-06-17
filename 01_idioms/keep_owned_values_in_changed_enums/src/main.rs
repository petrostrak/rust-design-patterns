// Say we have a &mut MyEnum which has (at least) two variants, A { name: String, x: u8 }
// and B { name: String }. Now we want to change MyEnum::A to a B if x is zero, while keeping
// MyEnum::B intact.
//
// We can do this without cloning the name.

use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // This takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will be
        // assigned to `*e`)
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}

fn main() {
    println!("Hello, world!");
}
