extern crate memory_arena;
use memory_arena::*;

#[derive(Debug)]
enum List<'a, T> {
    Cons(T, ArenaBox<'a, List<'a, T>>),
    Nil,
}

fn main() {
    let a = Arena::new(1024, 1024).unwrap();
    let list = a.new_box(List::Nil).unwrap();
    let list = a.new_box(List::Cons(1, list)).unwrap();
    let list = a.new_box(List::Cons(2, list)).unwrap();
    let list = a.new_box(List::Cons(3, list)).unwrap();
    println!("{:?}", list);
}
