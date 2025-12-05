pub struct IDRange {
    min: u64,
    max: u64,
}

impl IDRange {
    pub fn from_string(input: &str) -> Self {
        let strs: Vec<u64> = input
            .split("-")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Self {
            min: strs[0],
            max: strs[1],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.min..=self.max
    }
}

impl IntoIterator for &IDRange {
    type Item = u64;
    type IntoIter = std::ops::RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.min..=self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_instantiation() {
        let mut range = IDRange::from_string("0-1");
        assert_eq!(range.min, 0);
        assert_eq!(range.max, 1);

        range = IDRange::from_string("1-2");
        assert_eq!(range.min, 1);
        assert_eq!(range.max, 2);

        range = IDRange::from_string("1000-2000");
        assert_eq!(range.min, 1000);
        assert_eq!(range.max, 2000);

        range = IDRange::from_string("1-3");
        let mut range_iter = range.iter();
        assert_eq!(1u64, range_iter.next().unwrap());
        assert_eq!(2u64, range_iter.next().unwrap());
        assert_eq!(3u64, range_iter.next().unwrap());

        let mut expect_vals = Vec::<u64>::new();
        for i in &range {
            expect_vals.push(i);
        }
        assert_eq!(expect_vals, vec![1u64, 2u64, 3u64]);
    }
}
