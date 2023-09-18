fn main() {
    let mut s1 = String::from("Hello");
    let s2 = String::from("World");
    let new_string = concatinate_strings(&mut s1, &s2);
    println!("new string: {}", new_string);
    //s1 and new_string are not the same refrence 
    s1.push_str("!");
    println!("new string: {}", s1);
}

fn concatinate_strings(s1: &mut String, s2: &String) -> String {
    s1.push_str(&s2);
    return s1.to_string();
}
