struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_true() {
        assert!(true);
    }

    #[test]
    fn given_example_1() {
        assert_eq!(Solution::can_complete_circuit([1,2,3,4,5].to_vec(), [3,4,5,1,2].to_vec()), 3)
    }

}