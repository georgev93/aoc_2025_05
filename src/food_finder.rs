use crate::food::IDRange;

pub struct FoodFinder {
    fresh_ranges: Vec<IDRange>,
}

impl FoodFinder {
    pub fn new() -> Self {
        Self {
            fresh_ranges: Vec::new(),
        }
    }

    pub fn insert_id_range(&mut self, new_range: &str) {
        self.fresh_ranges.push(IDRange::from_string(new_range));
    }

    pub fn is_fresh(&self, id: &u64) -> bool {
        for id_range in &self.fresh_ranges {
            if id_range.into_iter().contains(id) {
                return true;
            }
        }
        false
    }

    pub fn sort_and_merge(&mut self) {
        self.fresh_ranges.sort_by_key(|idr| idr.min());

        let mut current_iter = self.fresh_ranges.iter();
        let mut merged = vec![*current_iter.next().unwrap()];

        for id_range in current_iter {
            let last_merged = merged.last_mut().unwrap();
            if last_merged.contains(id_range.min()) {
                if id_range.max() > last_merged.max() {
                    last_merged.set_max(id_range.max());
                }
                if id_range.contains(merged.last().unwrap().min()) {
                    merged.last_mut().unwrap().set_min(id_range.min());
                }
                // println!("Merging result: {:?}", merged.last().unwrap());
            } else {
                merged.push(*id_range);
            }
        }

        self.fresh_ranges = merged;
    }

    pub fn number_of_fresh_ids(&self) -> u64 {
        let mut ret_val = 0u64;
        for id_range in &self.fresh_ranges {
            ret_val += id_range.into_iter().size_hint().0 as u64;
        }
        ret_val
    }

    pub fn get_ranges(&self) -> Vec<(u64, u64)> {
        let mut ret_val = Vec::<(u64, u64)>::new();
        for id_range in &self.fresh_ranges {
            let new_element = (id_range.min(), id_range.max());
            ret_val.push(new_element);
        }
        ret_val
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hash() {
        let mut my_map = FoodFinder::new();
        my_map.insert_id_range("1-3");
        assert!(my_map.is_fresh(&1u64));
        assert!(my_map.is_fresh(&2u64));
        assert!(my_map.is_fresh(&3u64));
        assert!(!my_map.is_fresh(&4u64));
    }

    #[test]
    fn sorting_and_merging() {
        // low, high
        let mut my_map = FoodFinder::new();
        my_map.insert_id_range("1-3");
        my_map.insert_id_range("2-5");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // high, low
        my_map = FoodFinder::new();
        my_map.insert_id_range("2-5");
        my_map.insert_id_range("1-3");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // overlapping max, bigger first
        my_map = FoodFinder::new();
        my_map.insert_id_range("1-5");
        my_map.insert_id_range("2-5");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // overlapping max, bigger last
        my_map = FoodFinder::new();
        my_map.insert_id_range("2-5");
        my_map.insert_id_range("1-5");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // overlapping min, bigger first
        my_map = FoodFinder::new();
        my_map.insert_id_range("1-5");
        my_map.insert_id_range("1-3");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // overlapping min, bigger last
        my_map = FoodFinder::new();
        my_map.insert_id_range("1-3");
        my_map.insert_id_range("1-5");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // overlapping both, bigger last
        my_map = FoodFinder::new();
        my_map.insert_id_range("2-3");
        my_map.insert_id_range("1-5");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);

        // overlapping both, bigger first
        my_map = FoodFinder::new();
        my_map.insert_id_range("1-5");
        my_map.insert_id_range("2-3");
        my_map.sort_and_merge();
        assert_eq!(my_map.get_ranges(), vec![(1, 5)]);
    }
}
