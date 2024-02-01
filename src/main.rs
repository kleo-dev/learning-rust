use std::io;
mod sorting_algorithim;

fn main() {
    let mut input = String::new();
    loop {
        println!("What do you wan to run?\n[1] Sorting Algorithim");
        io::stdin().read_line(&mut input).expect("failed input");
        let _value = input.trim();
        if _value == "q" || _value == "quit" {
            break;
        } else if _value == "1" {
            sorting_algorithim::run();
        }
    }
}
