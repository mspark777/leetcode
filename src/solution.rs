pub struct Solution {}

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        let mut truck_size = truck_size;
        box_types.sort_unstable_by_key(|v| -v[1]);

        let mut result = 0;
        for box_type in &box_types {
            let count = box_type[0];
            let units = box_type[1];
            let min = truck_size.min(count);
            result += units * min;
            truck_size -= min;
            if truck_size <= 0 {
                break;
            }
        }

        result
    }
}
