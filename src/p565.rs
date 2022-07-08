#![allow(dead_code)]

use std::collections::HashSet;


fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut visited = HashSet::<usize>::new();
        let mut max = 0;
        for i in 0..nums.len() {
            println!("index: {} of {}", i, nums.len());
            let mut length = 0;
            let mut current = i;
            loop {
            println!("loop: {} ", current);
                if visited.contains(&current) {
                   max = std::cmp::max(max, length);
                   if max > nums.len() / 2 {
                       return max as i32;
                   }
                   break;
                } else {
                    visited.insert(current);
                    length += 1;
                    current = nums[current] as usize;
                }
            }
        }

        return max as i32;

    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn simple() {
        assert_eq!(Solution::array_nesting(vec!{0,1,2}), 1);
        assert_eq!(Solution::array_nesting(vec!{5,4,0,3,1,6,2}), 4);
        assert_eq!(Solution::array_nesting(vec![0,2,1]), 2);
    }




}