fn main()
{
    let s: Option<&str> = Some("hehe");

    if let Some(str) = s {
        println!("{}",str);
    }

    println!("{:?}",s);
}
