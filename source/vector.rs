fn main() {
    //let v:Vec<i32> = Vec::new();
    //
    //let mut v=Vec::new();
    //v.push(5);
    let v = vec![1, 2, 3];

    let second = v[1]; //&v[1];

    println!("{}", second);
    for i in &v {
        println!("{}", i);
    }

    let nmsl = v.get(100);

    if let None = nmsl {
        println!("hehe");
    }
}
