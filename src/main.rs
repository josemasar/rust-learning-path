fn main() {
    // The `mut` keyword lets the variable be changed
    let mut a_number = 10;
    let a_boolean = true;

    println!("the number is {}", a_number);
    println!("the boolean is {}", a_boolean);

    a_number = 15;

    println!("Mutability: the number is {}", a_number);

    // Declare second variable binding, shadows existing variable "b_number" 
    let b_number = 5;
    let b_number = b_number * 5;
    
    println!("Shadowing: b_number is {}", b_number);

    //Starting with Types
    // let x = 2.0;
    // let y: f32 = 3.0; 

    let is_bigger = 1 > 4;
    println!("{}", is_bigger);

    //char, String, &str
    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld");
    println!("{}", hello);

    //Tuples
    let tuple = ("hello", 5 , 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.2, 'c');

    //Structures
    struct Person {
        name: String,
        age: u8,
        likes_organges: bool
    }

    struct Point2D(u32,u32);

    struct Unit;

    let person = Person {
        name: String::from("JM"),
        likes_organges: true,
        age: 37
    };

    println!("I´m {}, my age is {} and it´s {} I like oranges", person.name, person.age, person.likes_organges);

    let some_coordinates = Point2D(5,2);
    println!("{} {}", some_coordinates.0, some_coordinates.1)

}
