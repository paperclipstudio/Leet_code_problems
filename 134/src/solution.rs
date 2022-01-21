struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for i in 0..gas.len() {
            let starting_station = i;
            if Solution::can_finish_from(i, &gas, &cost) {
                return i as i32;
            }
        }
        return -1;
    }

    pub fn can_finish_from(start:usize, gas:&Vec<i32>, cost: &Vec<i32>) -> bool{
        let mut current = start;
        let mut current_gas = 0;
        loop {
            current_gas += gas[current];
            current_gas -= cost[current];
           current = (current + 1) % gas.len();
           if current_gas < 0 || current == start {
               break;
           }
        } 
        return current == start && current_gas >= 0;
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
        
        assert!(!Solution::can_finish_from(0, &[1,2,3,4,5].to_vec(), &[3,4,5,1,2].to_vec()));
        assert!(Solution::can_finish_from(3, &[1,2,3,4,5].to_vec(), &[3,4,5,1,2].to_vec()));
        assert_eq!(Solution::can_complete_circuit([1,2,3,4,5].to_vec(), [3,4,5,1,2].to_vec()), 3);
    }

    #[test]
    fn edge() {
        assert!(Solution::can_finish_from(0, &[1].to_vec(), &[1].to_vec()))
    }

    #[test]
    fn example_2() {
        let gas = [2,3,4].to_vec();
        let cost = [3,4,3].to_vec();

        assert!(!Solution::can_finish_from(0,&gas, &cost));
        assert!(!Solution::can_finish_from(1,&gas, &cost));
        assert!(!Solution::can_finish_from(2,&gas, &cost));
    }

    #[test]
    fn hard_last_jump() {
        let gas = [2,3,4].to_vec();
        let cost = [15,1,1].to_vec();

        assert!(!Solution::can_finish_from(0,&gas, &cost));
        assert!(!Solution::can_finish_from(1,&gas, &cost));
        assert!(!Solution::can_finish_from(2,&gas, &cost));
    }



}