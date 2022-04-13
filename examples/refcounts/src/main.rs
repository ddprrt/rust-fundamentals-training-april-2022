use std::{rc::{Rc, Weak}, cell::RefCell, thread, sync::Arc};

#[derive(Debug)]
struct Person {
    name: String,
    orders: RefCell<Vec<Weak<Order>>>
}

#[derive(Debug)]
struct Order {
    id: u32,
    person: Rc<Person>
}

fn main() {
    let me = Person {
        name: "Stefan".to_string(),
        orders: RefCell::new(vec![])
    }; // original memory
    let me = Rc::new(me); // RC: 1

    let me_clone = me.clone(); // RC: 2, same data
    let me_clone_2 = me_clone.clone(); // RC: 3

    let order_1 = Order {
        id: 1,
        person: me_clone
    };


    let order_2 = Order {
        id: 1,
        person: me_clone_2
    };

    let order_1 = Rc::new(order_1);

    me.orders.borrow_mut().push(Rc::downgrade(&order_1));

    println!("{}", Rc::strong_count(&me));
    println!("{}", Rc::weak_count(&order_1));


    println!("{:?}", me);
    
    drop(me); // RC: 2


    //println!("{}", order_1.person.name);
    //drop(order_1); // RC: 1
    println!("{}", order_2.person.name);
    drop(order_2); // RC: 0 --> Free memory


    let some_data = vec![1, 2, 3, 4];
    let some_data = Arc::new(some_data);

    for _i in 0..3 {
        let some_data = some_data.clone();
        thread::spawn(move || {
            println!("{:?}", some_data);
        });
    }

    panic!("Oh no");
}
