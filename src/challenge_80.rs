use std::collections::HashMap;

// Otherwise it will complain. How I loathe these leetcode Solution objects
pub struct Solution {}

pub fn create_counter(nums: &mut Vec<i32>) -> HashMap<i32, usize> {
    // Create a counter hashmap for tracking counts of values
    let mut counter: HashMap<i32, usize> = HashMap::new();
    
    for num in nums {
        let count = counter.entry(*num).or_insert(0);
        *count += 1;
    }
    
    counter
}

pub fn empty_on_pred(x: HashMap<i32, usize>) -> HashMap<i32, usize> {
    // Only keep items in the hashmap that have values > 2, since that's 
    // what we ultimately want to remove from the list
    return x.into_iter()
        .filter(|&(_, v)| v > 2)
        .collect();
}



impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0i32
        } else {
            ();
        }
        
        let my_map = create_counter(nums);
        let mut my_map = empty_on_pred(my_map);
        let mut i: usize = 0;
        
        while i != nums.len() {
            let increment = {
                // Check (mutably) if the key is in the counter
                // if it is, and the count is > 3, decrement and remove
                // otherwise, remove from the hashmap entirely, since the count should
                // now be 2
                if let Some(val) = my_map.get_mut(&nums[i]) {
                    if *val == 3 {
                        let _ = my_map.remove(&nums[i]);
                    } else {
                        *val -= 1;
                    }
                    let _ = nums.remove(i);
                    // don't increment index, nums has decreased in length, so we stay at
                    // our current spot
                    0
                } else {
                    // otherwise, increment the index, we haven't mutated the vec
                    1
                }
            };
        
            i += increment;
        }
        
        nums.len() as i32
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unit_test() {
        let mut my_array = vec![1,1,1,2,2,3];
        let new_len = Solution::remove_duplicates(&mut my_array);
        assert_eq!(new_len, 5);
        assert_eq!(my_array, [1, 1, 2, 2, 3]);
    }

    #[test]
    fn empty_test() {
        let mut my_array = vec![];
        let new_len = Solution::remove_duplicates(&mut my_array);
        assert_eq!(new_len, 0);
        assert_eq!(my_array, vec![]);
    }
}   
