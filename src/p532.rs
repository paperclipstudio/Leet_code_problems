#![allow(dead_code)]

use std::collections::HashSet;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k:i32) -> i32 {
    //     let mut count = HashMap::new();
        let mut result = 0;

    //     for num in  nums {
    //         count.insert(num, count.get(&num).unwrap_or(&0) + 1);
    //     }
    //     for num in count.keys() {
    //         result += num * count.get(&(num+k)).unwrap_or(&0);
    //     }
    //     return result;
    // }


        let mut set = HashSet::new();

        

        if k == 0 {
            let mut set_double = HashSet::new();
            for num in nums {
                if !set.insert(num) {
                    set_double.insert(num);
                }
            }
            return set_double.len() as i32;
        } else {
            for num in nums {
                set.insert(num);
            }
            let set = &set;
            for num in (set).into_iter() {
                if set.contains(&(num + k)) {result += 1}
            }
        }
    return result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn basic() {
        assert_eq!(Solution::find_pairs(vec!{3,1,4,1,5}, 2), 2);
        // 1 1 3 4 5
    }


    #[test]
    fn basic2() {
        assert_eq!(Solution::find_pairs(vec![1,2,3,4,5], 1), 4);
    }

    #[test]
    fn k_is_zerp() {
        assert_eq!(Solution::find_pairs(vec![1,3,1,5,4], 0), 1);
    }

    // #[test]
    // fn many_to_many() {
    //     assert_eq!(Solution::find_pairs(vec![1,1,3,3,3], 2), 6);
    // }

}