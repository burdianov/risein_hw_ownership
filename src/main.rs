fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result: String = String::new();
    result.push_str(a);
    result.push_str(b);
    result
}

fn main() {
    let string1: String = String::from("First string. ");
    let string2: String = String::from("Second string.");

    let concatenated_string: String = concatenate_strings(&string1, &string2);

    println!("{concatenated_string}");
}
