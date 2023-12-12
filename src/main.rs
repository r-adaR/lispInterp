use std::io;
//use std::collections::HashMap;

fn eval<'a>(command: String, alist: &mut Vec<String>) -> String {
    let keyc = command.trim_end().to_string();
    
    // Eval on expressions
    if keyc.starts_with('(') {

    }
    
    // Eval on atoms
    let result = eval_atom(keyc, alist);
    return result;
}


fn eval_atom(atom: String, alist: &mut Vec<String>) -> String {
    
    // Check if the atom is a T, NIL, or a string.
    let upper_check = atom.to_uppercase();
    if upper_check == "T" || 
        upper_check == "NIL" ||
        atom.starts_with('\"') { return atom; }

    // Check if the atom is numeric.
    if let Ok(_num) = atom.parse::<f32>() {
        return atom;
    }
    
    return atom;
}


fn main() {
    // Declare association table as mutable hashmap.
    let mut alist: Vec<String> = Vec::new();
    
    // Loop forever to continuously take commands.
    loop {
        let mut command: String = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                let result: String = eval(command, &mut alist);
                println!("{}", result);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
