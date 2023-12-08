use std::io;
use std::collections::HashMap;

fn eval(command: String, atable: &mut HashMap<String, String>) {
    atable.insert(command, "value".to_string());
    println!("{}", atable.get("ok").expect("Value not found.").to_string() );
}

fn main() {
    // Declare association table as mutable hashmap.
    let mut atable: HashMap<String, String> = HashMap::new();
    
    // Loop forever to continuously take commands.
    loop {
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                eval(command, &mut atable);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
