fn main() {
    let string1: &str = "Hello";
    let string2: &str = " world";

    let concatenated_string: String = concatenate_strings(string1, string2);
    println!("Concatenated string: {}", concatenated_string);
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result: String = "".to_string();
    result.push_str(s1);
    result.push_str(s2);
    return result;
}
