mod stacks;
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

fn is_valid(s: String) -> bool {
    let mut stack = vec!['0'];
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop().unwrap() != '(' || stack.is_empty() {
                    return false;
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' || stack.is_empty() {
                    return false;
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' || stack.is_empty() {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.len() == 1
}

fn is_valid2(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                let res = || "([{".find(top) == ")]}".find(c);
                return res();
            }
            _ => (),
        }
    }
    stack.len() == 0
}
