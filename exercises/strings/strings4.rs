// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!





fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string("blue".to_string()); // "blue" 是 &str，需要转为 String
    string_slice(&"red".to_string()); // "red" 转换为 String 后再转为 &str
    string(String::from("hi"));
    string("rust is fun!".to_owned()); // to_owned() 返回一个 String，&String 可被解引用为 &str
    string_slice("nice weather".into()); // into() 返回一个 String，并转换为 &str
    string(format!("Interpolation {}", "Station")); // format! 返回 String
    string_slice(&String::from("abc")[0..1]); // 取 "abc" 的前一个字符作为 &str
    string_slice("  hello there ".trim()); // trim() 返回 &str
    string_slice("Happy Monday!".to_string().replace("Mon", "Tues").as_str()); // replace() 返回 String，as_str() 将 String 转为 &str
    string_slice("mY sHiFt KeY iS sTiCkY".to_lowercase().as_str()); 
}
