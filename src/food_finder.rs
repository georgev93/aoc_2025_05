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
}
