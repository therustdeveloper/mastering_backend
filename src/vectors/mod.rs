pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("--- Vectors ---");

        definition::master(false);
    }
}