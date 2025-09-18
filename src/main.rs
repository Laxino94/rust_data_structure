mod stacks;
fn main() {
    println!("Hello, world!");
    let s = String::from("({[][]})");
    let res = is_valid(&s);
    println!("res: {}", res);
    let res = is_valid2(&s);
    println!("res: {}", res);
    let num = -100;
    let res = convert_to_base7(num);
    println!("res: {}", res);
    let token = [
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ];

    // assert_eq!(eval_rpn(token.iter().map(|s| s.to_string()).collect()), 22);
    println!(
        "result: {}",
        eval_rpn(token.iter().map(|s| s.to_string()).collect())
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

fn is_valid(s: &String) -> bool {
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

fn is_valid2(s: &String) -> bool {
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

fn convert_to_base7(mut num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut res = String::new();
    let is_negative = num < 0;
    if is_negative {
        num = -num;
    }
    while num > 0 {
        let remain = num % 7;
        // b'0' is the ASCII value for char '0'
        res.push((b'0' + remain as u8) as char);
        num /= 7; // quotient will finally be 0
    }
    if is_negative {
        res.push('-');
    }
    res.chars().rev().collect()
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut nums = Vec::new();
    for token in tokens.into_iter() {
        match token.parse() {
            Ok(n) => nums.push(n),
            Err(_) => {
                let r = nums.pop().unwrap();
                let l = nums.pop().unwrap();
                let res = do_op(&token, l, r);
                nums.push(res);
            }
        }
    }
    nums[0]
}

fn do_op(op: &str, a: i32, b: i32) -> i32 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        _ => a / b,
    }
}
