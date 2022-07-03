#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn basic() -> i32 {
        return 42;
    } 
    pub fn parse_bool_expr(expression: String) -> bool {
        let expression = expression.as_str();
        return Solution::parse_bool(expression);
    }

    pub fn remove_brackets(str:&str) -> &str {
        return str.strip_suffix(')').unwrap().strip_prefix('(').unwrap();
    }

    
    

    pub fn parse_bool(expression: &str) -> bool {
        let mut balance = 0;
        let balance_split = |c: char| {
            if c == '(' { balance += 1};
            if c == ')' { balance -= 1};
            c == ',' && balance == 0
        };
        return match expression.chars().nth(0).unwrap_or('@') {
            't' => true,
            'f' => false,
            '!' => !(Self::parse_bool(
                Self::remove_brackets(expression.strip_prefix('!').unwrap()))),
            '&' => {
                Self::remove_brackets(expression.strip_prefix('&').unwrap())
                    .split(balance_split)
                    .map(|expr| Self::parse_bool(expr))
                    .all(|x|x)
            },
            '|' => {
                Self::remove_brackets(expression.strip_prefix('|').unwrap())
                    .split(balance_split)
                    .map(|expr| Solution::parse_bool(expr))
                    .any(|x|x)
            }

            _ => panic!("Fail to parse {}", expression)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
   fn life_universe_everything() {
       assert_eq!(Solution::basic(), 42);
   }

    #[test]
   fn t() {
       assert_eq!(Solution::parse_bool_expr(String::from("t")), true);
   }

    #[test]
   fn f() {
       assert_eq!(Solution::parse_bool_expr(String::from("f")), false);
   }



    #[test]
   fn not() {
       assert_eq!(Solution::parse_bool_expr(String::from("!(t)")), false);
       assert_eq!(Solution::parse_bool_expr(String::from("!(f)")), true);
   }

    #[test]
   fn and() {
       assert_eq!(Solution::parse_bool_expr(String::from("&(t,f)")), false);
       assert_eq!(Solution::parse_bool_expr(String::from("&(t,t)")), true);
       assert_eq!(Solution::parse_bool_expr(String::from("&(f,f)")), false);
   }

     #[test]
   fn or() {
       assert_eq!(Solution::parse_bool_expr(String::from("|(t,f)")), true);
       assert_eq!(Solution::parse_bool_expr(String::from("|(t,t)")), true);
       assert_eq!(Solution::parse_bool_expr(String::from("|(f,f)")), false);
   }

   #[test]
   fn splits() {
       //assert_eq!(Solution::split("&(t,f), f").as_slice(), ["&(t,f)", "f"])
   }


   #[test]
   fn test_cases() {
       assert_eq!(Solution::parse_bool_expr(String::from("|(&(t,f,t),!(t))")), false);

   }
}