struct Person(i32);

fn main() {
    let mut vec: Vec<Person> = vec![Person(10), Person(20)];

    let vec_ref = &mut vec;
    for i in vec_ref {
        i.0 = i.0 * 2;
    }

    for i in vec {
        println!("{}", i.0);
    }

}
