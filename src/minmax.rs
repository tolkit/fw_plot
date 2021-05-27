pub mod minmax {

    // thanks https://users.rust-lang.org/t/how-to-get-min-max-min-index-max-index/45324
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct MaxMin<T> {
        pub max: T,
        pub min: T,
    }

    pub fn find_max_min<T: std::cmp::PartialOrd + Copy>(slice: &[T]) -> MaxMin<T> {
        let mut max = &slice[0];
        let mut min = &slice[0];

        for index in 1..slice.len() {
            if slice[index] < *min {
                min = &slice[index];
            }
            if slice[index] > *max {
                max = &slice[index];
            }
        }

        MaxMin {
            max: *max,
            min: *min,
        }
    }
}
