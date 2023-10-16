struct hehe{
    data: String,
}

impl Drop for hehe {
    fn drop(&mut self) {
        println!("{} G",self.data);
    } 
}

fn main(){
    let s1 = hehe{
        data: String::from("hehe"),
    };

    let s2 = hehe{
        data: String::from("haha"),
    };

    
}
