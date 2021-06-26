// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    let word = match optional_word {
        Some(word) => format!("The word is: {}", word),
        None => format!("The optional word doesn't contain anything")
    };

    println!("Word is {}", word);

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..=10 {
        optional_integers_vec.push(Some(x));
    }

    optional_integers_vec.push(None);

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while optional_integers_vec.len() > 0 {
        
        let integer = optional_integers_vec.pop();
        println!("current value: {:?}", integer.unwrap());
    }
}
