use ahash::AHashSet;

use crate::food::IDRange;

pub struct FoodFinder {
    map: AHashSet<u64>,
}

impl FoodFinder {
    pub fn new() -> Self {
        Self {
            map: AHashSet::new(),
        }
    }

    pub fn insert_id_range(&mut self, new_range: &IDRange) {
        for id in new_range {
            println!("{id}");
            self.map.insert(id);
        }
    }

    pub fn is_fresh(&self, id: &u64) -> bool {
        self.map.contains(id)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hash() {
        let mut my_map = FoodFinder::new();
        my_map.insert_id_range(&IDRange::from_string("1-3"));
        assert!(my_map.is_fresh(&1u64));
        assert!(my_map.is_fresh(&2u64));
        assert!(my_map.is_fresh(&3u64));
        assert!(!my_map.is_fresh(&4u64));
    }
}
