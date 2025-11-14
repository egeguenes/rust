use std::rc::Rc;
fn main() {
    let pointer = Rc::new(1);
    {
        let second = pointer.clone();
        println!("{}", *second);
    }
    println!("{}", *pointer);
}
