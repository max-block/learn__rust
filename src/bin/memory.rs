#[allow(dead_code)]
fn try_drop() {
    let s = String::from("s1");
    drop(s);
    // println!("{}", s); Can't use s1, because value moves to drop(s)
}

#[allow(dead_code, unused_variables)]
fn drop_trait() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // CustomSmartPointers created.
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!
}

#[allow(dead_code, unused_variables)]
fn try_rc() {
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // count after creating a = 1
    // count after creating b = 2
    // count after creating c = 3
    // count after c goes out of scope = 2
}

fn main() {
    // try_drop();
    // drop_trait();
    // try_rc();
    try_ref_cell();
}
