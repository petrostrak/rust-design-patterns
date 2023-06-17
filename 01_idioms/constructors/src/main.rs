#[derive(Debug)]
pub struct First {
    value: u64,
}

impl First {
    // Constructs a new instance of ['First'].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug)]
pub struct Second {
    value: u64,
}

impl Second {
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 10 }
    }
}

fn main() {
    let first = First::new(5);
    let secord = Second::default();
    println!("{:?}", first);
    println!("{:?}", secord);
}
