

fn main() {
    println!("{}", reverse_it(i32::MIN));
    println!("{}", reverse_it(-123));
}

pub fn reverse_it(v: i32) -> String {
let mut negative = false;
if v < 0{
negative = true;
}
let mut test = String::new();
let mut res : String = v.to_string().chars().rev().collect();
if negative{
    test.push('-');
    test.push_str(&res);   
    test.pop().unwrap();
}


test
}