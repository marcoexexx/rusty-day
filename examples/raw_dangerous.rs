fn dangerously_access() {
    let addr;

    {
        let x = String::from("hello");
        addr = &x as *const String;
    }

    // It can still access value even drop owner, it ignore rust's ownership and borrow checker
    unsafe {
        println!("{}", *addr);
    }
}

fn main() {
    dangerously_access();
}
