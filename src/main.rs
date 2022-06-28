#![allow(dead_code)]

enum Person{
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info{name: String, age: i32}
}

fn inspect(p: Person){
    match p {
        Person::Engineer => println!("Is a Engineer!"),
        Person::Scientist => println!("Is a Scientist"),
        Person::Height(h) => println!("Has a height of {}", h),
        Person::Weight(w) => println!("Has a weight of {}", w),
        Person::Info{name, age} => println!("{} is {} years old", name, age),
    }
}

fn main() {
    let kev0 = Person::Height(180);
    let kev1 = Person::Weight(85);
    let kev2 = Person::Engineer;
    let kev3 = Person::Info{name:"Kev".to_owned(), age: 27};

    inspect(kev0);
    inspect(kev1);
    inspect(kev2);
    inspect(kev3);
}
