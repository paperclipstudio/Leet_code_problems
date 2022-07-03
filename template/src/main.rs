#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn basic() -> i32 {
        return 42;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
   fn life_universe_everything() {
       assert_eq!(Solution::basic(), 42);
   }



}