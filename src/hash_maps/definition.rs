use std::collections::HashMap;

pub fn master(show: bool) {
    if show {
        println!("Hash Maps Definition");

        create_hashmap();
    }
}

fn create_hashmap() {
    // create a new hashmap
    let mut data = HashMap::new();
    // insert data into this hashmap
    data.insert(String::from("one"), 1);
    data.insert(String::from("two"), 2);
    // check the content of the hashmap
    println!("Content of data: {:?}", data);

    // accessing value in HashMap
    if let Some(data) = data.get("one") {
        println!("The first data here is: {}", data);
    } else {
        println!("That data is not found");
    }

    // iterating over the HashMap
    for (key, value) in &data {
        println!("{} matches with {}", key, value);
    }

    // remove values from the HashMap
    data.remove("two");
    println!("Content of data: {:?}", data);
}