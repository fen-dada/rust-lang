fn main() {
    let s1 = String::from("hello");

    let l = calc(&s1);

    println!("{} len is {}", s1, l);
}

fn calc(s: &String) -> usize {
    s.len()
}
