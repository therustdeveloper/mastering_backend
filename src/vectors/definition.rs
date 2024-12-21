pub fn master(show: bool) {
    if show {
        create_vector();
        pass_all_data_types_to_vectors();
    }
}

fn create_vector() {
    // Create a new mutable vector of type i32
    let mut new_vector: Vec<i32> = Vec::new();
    // Print out the content of the vector
    println!("Vector content: {:?}", new_vector);
    // Push a value into the vector
    new_vector.push(1);
    // Print out the new content of the vector
    println!("Vector content: {:?}", new_vector);
}

fn pass_all_data_types_to_vectors() {
    // integer values
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in numbers.iter() {
        println!("The number is {}", i);
    }

    // flaoting point values
    let temperatures: Vec<f64> = vec![36.6, 27.0, 15.3];
    for i in temperatures.iter() {
        println!("The value of temperature is {}", i);
    }

    // strings
    let names: Vec<String> = vec!["Bob".to_string(), "Alice".to_string()];
    for name in names.iter() {
        println!("The value of name is {}", name);
    }

    // booleans
    let flags: Vec<bool> = vec![true, false, true, true];
    for flag in flags.iter() {
        println!("The value of flag is {}", flag);
    }

    // custom structs
    struct Person {
        name: String,
        age: u32,
    }

    let people: Vec<Person> = vec![
        Person {
            name: "Bob".to_string(),
            age: 27,
        },
        Person {
            name: "Alice".to_string(),
            age: 37,
        },
    ];

    // enums
    enum FilterOptions {
        Free,
        Paid,
        Bonus,
    }

    let filters: Vec<FilterOptions> = vec![
        FilterOptions::Free,
        FilterOptions::Paid,
        FilterOptions::Bonus,
    ];

    // mixed types using trait objects
    let mixed: Vec<Box<dyn std::fmt::Debug>> = vec![Box::new(42), Box::new("Alice".to_string())];
}