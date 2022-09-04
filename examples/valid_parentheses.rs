/* Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
determine if the input string is valid. Recursion*/
pub fn is_valid(s: String) -> bool {
    if s.len() == 0 {
        return true;
    } else {
        let mut i = 0;
        let mut br = 0;
        let mut left = 0;
        let mut right = 0;
        let mut op = '(';
        let mut cl = ')';
        while i < s.len() {
            if br == 0
                && (s.as_bytes()[i] as char == '('
                    || s.as_bytes()[i] as char == '{'
                    || s.as_bytes()[i] as char == '[')
            {
                if s.as_bytes()[i] as char == '(' {
                    op = '(';
                    cl = ')';
                } else if s.as_bytes()[i] as char == '{' {
                    op = '{';
                    cl = '}';
                } else if s.as_bytes()[i] as char == '[' {
                    op = '[';
                    cl = ']';
                }
                left = i;
                br += 1;
            } else if br == 0
                && (s.as_bytes()[i] as char == ')'
                    || s.as_bytes()[i] as char == '}'
                    || s.as_bytes()[i] as char == ']')
            {
                return false;
            } else if br > 0 && s.as_bytes()[i] as char == op {
                br += 1;
            } else if br > 0 && s.as_bytes()[i] as char == cl {
                br -= 1;
                if br == 0 {
                    right = i;
                    if !is_valid(s[left + 1..right].to_string()) {
                        return false;
                    }
                }
            }
            i += 1
        }
        if br > 0 {
            return false;
        }
        return true;
    }
}

fn main() {
    let s = "[()](())".to_string();
    println!("{:?}", is_valid(s));
}
