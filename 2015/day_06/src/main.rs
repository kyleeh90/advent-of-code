// Solution for https://adventofcode.com/2015/day/6

// Using HashSet for storing the coordinates and lights
use std::collections::HashSet;

// Create a struct to represent an individual light
struct Light{
    is_on: bool
}

// Create a Vector2 struct to represent coordinates
struct Vector2{
    x: u64,
    y: u64
}

impl Light{
    fn new() -> Light{
        Light{is_on: false}
    }

    fn toggle(&mut self){
        self.is_on = !self.is_on;
    }

    fn turn_off(&mut self){
        self.is_on = false;
    }

    fn turn_on(&mut self){
        self.is_on = true;
    }
}


fn main() {
    println!("Hello, world!");
}