#![allow(unused)]

use std::pin::Pin;

fn is_unpin<T: Unpin>(data: T) -> bool {
    true
}

fn dangerously_move() {
    let v = Box::new(String::from("hello"));
    println!("addr: {:p} -> {:p}", &v, &(*v));

    let addr = &*v as *const String;

    unsafe {
        println!("val: {:p} - {}", addr, *addr);
    }

    let _v = *v;
    println!("\nmoved: {:p}\n", &*_v);

    unsafe {
        println!("val: {:p} - {}", addr, *addr);
    }
}

fn prevent_move_on_stack() {
    let data = String::from("hello"); // `String` is movable.
    let pinned = Pin::new(&data); // Pinned reference, but doesn't truly prevent moves.

    // Correct usage: we cannot move `data` through `pinned`,
    // but `data` itself can still be moved by its own value.
}

trait Todo {}

struct Cat;

impl Todo for Cat {}

fn prevent_move_on_heap() {
    let data = Cat; // movable
                    // `Todo` trait is unknown size, so dyn allocate in heap
    let pinned: Pin<Box<dyn Todo>> = Box::pin(data);

    // compile error: cannot move
    // let moved = *pinned;
}

fn main() {
    // dangerously_move();
}
