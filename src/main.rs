fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);
}
fn first_word(t: &String) -> &str {
    &t[..1]
}
