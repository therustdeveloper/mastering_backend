pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("--- Hash Maps ---");

        definition::master(false);
    }
}