impl Solution {
    // Beats 33% on runtime, 100% on memory. Not sure how, since I'm holding a vec
    // of indices, but I'll take it. Will add Solution2 for better runtime, hopefully I can 
    // find a balance here
    
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
        // catch empty arrays, otherwise index-errors will be raised
        if nums.is_empty() {
            return 0i32
        } else {
            ();
        }

        // establish first value to check
        let mut val: i32 = nums[0];
        let mut indices: Vec<usize> = Vec::new();
        
        for i in (1..nums.len()) {
            let new_val = nums[i];
            if new_val == val {
                indices.push(i);
            } else {
                val = new_val;
            }
        }
        
        // go in reverse order so that the indices don't change values while
        // removing bad values
        for index in indices.iter().rev() {
            let _ = nums.remove(*index);
        }
        
        // Array length is the expected return, which is a usize IIRC
        nums.len() as i32
    }
}
