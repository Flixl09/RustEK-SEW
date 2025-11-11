// src/lib.rs

extern crate core;

use rand::Rng;

// ---------- 1. BASICS ----------
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn flip(b: bool) -> bool {
    !b
}

// ---------- 2. OWNERSHIP & BORROWING ----------
pub fn take_ownership(s: String) -> usize {
    // TODO: return the length of s (taking ownership)
    // hint: no borrowing here; s moves i
    let string = s;
    string.len()
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    // TODO: return the first char without taking ownership
    s.chars().next()
}

pub fn push_exclamation(s: &mut String) {
    // TODO: mutate s by appending a single '!' character
    s.push('!');
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        // TODO: Euclidean distance
        let distance_x = (self.x - other.x).abs();
        let distance_y = (self.y - other.y).abs();

        (distance_x.powf(2.0) + distance_y.powf(2.0)).sqrt()
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        // TODO: match on self, compute area
        match self {
            Shape::Circle { center: _, radius } => radius.powf(2.0) * std::f64::consts::PI,
            Shape::Rect {top_left: _, w, h} => w * h
        }
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}

// Return a reference to the item farthest from the origin.
// Note the explicit lifetime tying the returned reference to the input slice.
pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    items.iter().max_by(
        |x,y|
            (x.x().powf(2.0) + x.y().powf(2.0))
                .total_cmp(
                    &(y.x().powf(2.0) + y.y().powf(2.0))))
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    // TODO: parse string into u16; map errors into a friendly String
    // hint: use s.parse::<u16>()
    let port = s.parse::<u16>();
    let port_parsed: Result<u16, String> = port.map_err(|err| err.to_string());
    port_parsed
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    // TODO: all even numbers from 0..=n, squared, collected to Vec
    // hint: (0..=n).filter(...).map(...).collect()
    (0..=n).filter(|x| x % 2 == 0).map(|x| x.pow(2)).collect()
}

// ---------- 7. USING A CRATE (rand) ----------
pub fn roll_dice(sides: u8) -> u8 {
    // TODO: return a value in 1..=sides using rand::Rng
    // note: assume sides >= 1
    rand::rng().random_range(1..=sides)
}
