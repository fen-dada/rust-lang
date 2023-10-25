//use Post;

fn main() {
    let mut post = Post::new();

    post.add_text("hehe");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("hehe",post.content());
}
