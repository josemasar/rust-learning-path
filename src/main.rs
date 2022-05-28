fn divide_by_2(dividend: u32, divisor: u32) -> bool {
    if dividend == 0 {
        return false;
    }
    dividend % divisor == 0
}



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

    // struct Unit;

    let person = Person {
        name: String::from("JM"),
        likes_organges: true,
        age: 37
    };

    println!("I´m {}, my age is {} and it´s {} I like oranges", person.name, person.age, person.likes_organges);

    let some_coordinates = Point2D(5,2);
    println!("{} {}", some_coordinates.0, some_coordinates.1);

    //Enums

    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);
    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick { x: i64, y: i64 }

    // Define the WebEvent enum variants to use the data from the structs
    // and a boolean type for the page Load variant
    #[derive(Debug)]
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);
        
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
        
    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
        
    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

    //Calling a function

    assert_eq!(divide_by_2(4, 2), true);

    //Matrix

    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    println!("Selected day: {}", days[1]);

    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];
    println!("Byte content:{}", bytes[1]);

    //Vectors

    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes); 

    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();
    fruit.push("Apple");
    println!("Fruits: {:?}", fruit)


    //HashMaps



}
