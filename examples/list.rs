extern crate memory_arena;
use memory_arena::*;

#[derive(Debug)]
enum List<'a, T> {
    Nil,
    Cons(T, ArenaBox<'a, List<'a, T>>),
}
//TODO: figure out why swapping Nil and Cons results in a segfault
// when trying to write the value to the ArenaBox's pointer

fn main() {
    let a = Arena::new(1024, 1024).unwrap();
    let list = a.new_box(List::Nil).unwrap();
    let list = a.new_box(List::Cons(1, list)).unwrap();
    let list = a.new_box(List::Cons(2, list)).unwrap();
    let list = a.new_box(List::Cons(3, list)).unwrap();
    println!("{:?}", list);
}
