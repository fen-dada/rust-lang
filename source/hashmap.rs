use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    let i = 345;

    hm.insert(String::from("Blue"), 10);

    hm.insert(String::from("Yellow"), 50);

    let team = String::from("Blue");
    let score = hm.get(&team).copied().unwrap_or(0); // if there's no such score, then it'll be 0

    println!("{}", score);

    for (key, value) in &hm {
        println!("{key}:{value}");
    }

    hm.entry(String::from("Blue")).or_insert(100); // if no (Blue,100) the insert it.
}
