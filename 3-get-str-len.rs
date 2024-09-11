fn main() {
    let name = String::from("harkirat");
    let len = get_str_len(name);
    println!("the length of the string is {}", len);
}

fn get_str_len(str: String) -> usize {
    return str.chars().count();
}

