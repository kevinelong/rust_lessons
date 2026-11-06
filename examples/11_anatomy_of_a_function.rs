//! Section 11a, running: the anatomy of a function, one token at a time.
//!
//! `cargo run --example 11_anatomy_of_a_function`
//!
//! A function declaration is a sentence with a fixed grammar. This example takes
//! one apart in front of you, then builds the same function four other ways — as an
//! associated function, a method, a closure, and a function that returns nothing.

#![allow(clippy::items_after_statements)]

/// The specimen, with every part named in the comments above it.
///
/// ```text
/// pub  fn  describe_dog  (name: &str, age: u8)  ->  String  {  format!(...)  }
/// │    │   │              │                     │   │       │  │
/// │    │   │              │                     │   │       │  └ the body, an expression
/// │    │   │              │                     │   │       └ opening brace
/// │    │   │              │                     │   └ return type
/// │    │   │              │                     └ arrow: "returns"
/// │    │   │              └ parameter list: a pattern, a colon, a type
/// │    │   └ name, snake_case
/// │    └ keyword: this is a function
/// └ visibility: callable from outside this module
/// ```
pub fn describe_dog(name: &str, age: u8) -> String {
    // The last expression is the return value. No `return`, no semicolon:
    // adding one would turn this into a statement and the function into `()`.
    format!("{name} is {age}")
}

/// A function that returns nothing: the return type is the unit type `()`.
pub fn log_only(message: &str) {
    let _ = message; // `_` is "I am deliberately not binding this"
}

/// A function whose body never finishes normally returns `!`, the never type.
///
/// Not called by this example — it exists so the signature can be read out loud:
/// `fn abort() -> !` means "this never gives control back".
#[allow(dead_code)]
pub fn abort_forever() -> ! {
    loop {
        std::thread::park();
    }
}

/// A struct so the associated-function and method forms can be shown.
pub struct Dog {
    pub name: String,
}

impl Dog {
    /// An **associated function**: no `self`, called as `Dog::new(...)`.
    /// This is the constructor idiom — Rust has no `new` keyword.
    pub fn new(name: &str) -> Self {
        Dog {
            name: name.to_string(),
        }
    }

    /// A **method**: takes `&self`, called as `dog.describe()`. The `&` means
    /// "borrows the dog for the call" rather than consuming it.
    pub fn describe(&self) -> String {
        format!("{} is a dog", self.name)
    }

    /// A method that changes the value, so it needs `&mut self`.
    pub fn rename(&mut self, to: &str) {
        self.name = to.to_string();
    }
}

fn main() {
    println!("rust_lessons {} — section 11a", rust_lessons::VERSION);

    println!();
    println!("  1. the parts of a signature");
    for (part, meaning) in [
        ("pub", "visible outside this module"),
        ("fn", "the keyword that starts a function"),
        ("describe_dog", "the name, snake_case"),
        ("(name: &str, age: u8)", "parameters: pattern, colon, type"),
        ("-> String", "the type of the value returned"),
        ("{ ... }", "the body, whose last expression is the value"),
    ] {
        println!("    {part:<24} {meaning}");
    }

    println!();
    println!("  2. calling it");
    println!("    describe_dog(\"Rex\", 3) -> {}", describe_dog("Rex", 3));
    log_only("a function with no return value");
    println!("    log_only(...) -> () (nothing to print)");

    println!();
    println!("  3. associated function vs method");
    let mut dog = Dog::new("Rex");
    println!("    Dog::new(\"Rex\")      associated, no self   -> {}", dog.describe());
    dog.rename("Rex the Second");
    println!("    dog.rename(..)       method, &mut self     -> {}", dog.describe());

    println!();
    println!("  4. the same job as a closure: |age| age > 2");
    let is_old = |age: u8| age > 2;
    for age in [1, 3] {
        println!("    is_old({age}) = {}", is_old(age));
    }

    println!();
    println!("  5. functions as values, which is why `|` exists");
    let describe = describe_dog;
    println!("    describe(\"Ada\", 4) -> {}", describe("Ada", 4));

    println!();
    println!("  Symbols used above, and where the course introduces each:");
    for (symbol, meaning, section) in [
        ("fn", "a function declaration", "11"),
        ("()", "parameter list, and the unit type", "11"),
        ("->", "returns", "11"),
        ("&", "borrow a reference", "04"),
        ("&mut", "borrow, allowed to change", "13"),
        ("::", "path into a type or module", "06"),
        ("|x|", "a closure and its parameters", "09"),
        ("_", "deliberately unused", "11"),
        ("!", "never type; also the macro marker", "11"),
    ] {
        println!("    {symbol:<6} {meaning:<38} section {section}");
    }
}
