fn main() {
    let ans;

    let str1 = String::from("small");
    {
        let str2 = String::from("longer");

        ans = longest(&str1, &str2);
    }

    println!("{}", ans);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

