/* Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
determine if the input string is valid. Stack*/
pub fn is_valid(s: String) -> bool {
    let mut buf = Vec::new();
        for p in s.chars() {
            if p == '(' || p == '{' || p == '[' {
                buf.push(p);
            } else {
                if buf.is_empty() {
                    return false;
                }
                let last = buf.pop().unwrap();
                if (p == ')' && last != '(') || (p == '}' && last != '{') || (p == ']' && last != '[') {
                    return false;
                }
            }
        }
        if buf.len() != 0 {
            return false;
        }
        return true;
}

fn main() {
    let s = "[()](())".to_string();
    println!("{:?}", is_valid(s));
}
