use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complement_to_index: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.into_iter().enumerate() {
        let complement = target - num;
        match complement_to_index.get(&complement) {
            Some(&complement_index) => {
                return vec![complement_index as i32, index as i32];
            }
            None => {
                complement_to_index.insert(num, index);
            }
        }
    }

    vec![-1, -1]
}
