extern crate memory_arena;
use memory_arena::*;

#[derive(Debug)]
enum Choice {
    A,
    B,
}

fn main() {
    let arena = Arena::new(1024, 1024).unwrap();
    let a = arena.new_box(Choice::A).unwrap();
    let b = arena.new_box(Choice::B).unwrap();
    println!("{:?}", a);
    println!("{:?}", b);
}
