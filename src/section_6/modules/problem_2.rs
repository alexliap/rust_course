mod m1 {

    struct A {
        d: m2::D,
    }

    pub mod m2 {
        pub enum D {
            B,
            C,
        }
    }
}

mod m3 {
    use crate::m1::m2::D;

    struct C {
        e: D,
    }
}

fn main() {}
