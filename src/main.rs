fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(" World!");
    println!("{}", concatenate_strings(&s1, &s2));
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}
