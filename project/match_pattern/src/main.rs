fn main(){
    let favorite_color: Option<&str> = Some("hehe");

    let is_tuesday = false;

    let age: Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
         println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let v = vec![1,2,3];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}",value,index);
    }

    println!("{:?}",favorite_color);
}
