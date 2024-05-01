use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use crate::CircularList::{CircularCons, CircularNil};
use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RefCellList::{RefCellCons, RefCellNil};

fn main() {
    // box example
    let b = Box::new(5);
    println!("b = {}", b);

    // box example with recursive List type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);

    // dereferencing a reference
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // dereferencing a Box
    let y = Box::new(x);
    assert_eq!(5, *y);

    // dereferencing a custom type
    let y = MyBox::new(x);
    assert_eq!(5, *y);

    // dereference coersion
    let m = MyBox::new(String::from("Jack"));
    // m is a reference to the Box, which is dereferenced to get the String
    // which is dereferenced to get the &str
    hello(&**m);
    // m is the same, but the Box dereference is implicit
    hello(&m);

    // Drop examples
    let c = CustomSmartPointer {
        data: String::from("some stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // Manualy calling Drop
    // c.drop();
    drop(c);
    println!("c dropped before the end of main");

    // Reference Counting list
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("Counter after creating A: {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("Counter after creating B: {}", Rc::strong_count(&a));

    {
        let c = RcCons(4, Rc::clone(&a));
        println!("Counter after creating C: {}", Rc::strong_count(&a));
        println!(
            "Rc list A: {:?}\n Rc list B: {:?}\n Rc list C: {:?}",
            a, b, c
        );
    }
    println!(
        "Counter after C goes out of scope: {}",
        Rc::strong_count(&a)
    );

    // RefCellList examples
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCellCons(Rc::clone(&value), Rc::new(RefCellNil)));
    let b = RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let a = Rc::new(CircularCons(5, RefCell::new(Rc::new(CircularNil))));

    println!("a's initial rc count = {}", Rc::strong_count(&a));
    println!("a's next item = {:?}", a.tail());

    let b = Rc::new(CircularCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b's creation: {}", Rc::strong_count(&a));
    println!("b's initial rc count = {}", Rc::strong_count(&b));
    println!("b's next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b's rc count after changing a: {}", Rc::strong_count(&b));
    println!("a's rc count after changing a: {}", Rc::strong_count(&a));

    // Uncomment this line to see that we have a cycle
    // it will overflow the stack lol
    // println!("a's next item: {:?}", a.tail());

    // Tree example
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, branch weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// List implemented with Boxes
// only one reference at a time, immutable or mutable borrows
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// reference counting List
// can have multiple references at a time, all immutable
#[derive(Debug)]
enum RcList<T> {
    RcCons(T, Rc<RcList<T>>),
    RcNil,
}

// RefCellList contains an RC that holds a RefCell
// This way, we can have multiple references to the same list
// and we can have mutability
#[derive(Debug)]
enum RefCellList<T> {
    RefCellCons(Rc<RefCell<T>>, Rc<RefCellList<T>>),
    RefCellNil,
}

#[derive(Debug)]
enum CircularList<T> {
    CircularCons(T, RefCell<Rc<CircularList<T>>>),
    CircularNil,
}

impl<T> CircularList<T> {
    fn tail(&self) -> Option<&RefCell<Rc<CircularList<T>>>> {
        match self {
            CircularCons(_, item) => Some(item),
            CircularNil => None,
        }
    }
}

// custom Box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// this allows me to &MyBox to get a ref to the value inside
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// custom smart pointer to print when dropped
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

// Weak Rc's to create a tree
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
