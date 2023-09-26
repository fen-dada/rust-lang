use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement : {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn getStr(x: &str) -> String {
    let result = String::from("nmsl");

    result
}

fn main() {
    let str1 = String::from("111nmsl");

    {
        let str2 = String::from("zz");
        let result = longest(&str1, &str2);

        println!("longest is {}", result);
    }

    let result = getStr(&str1);

    println!("{}", result);

    let hehe = longest_with_announcement(&str1, "111", 3);

    println!("{}", hehe);
}
