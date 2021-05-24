pub mod vec2arr {
    use ndarray::prelude::*;

    // https://github.com/rust-ndarray/ndarray/issues/590
    pub fn vec_to_array<T: Clone>(v: Vec<Vec<T>>) -> Array2<T> {
        if v.is_empty() {
            return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
        }
        let nrows = v.len();
        let ncols = v[0].len();
        let mut data = Vec::with_capacity(nrows * ncols);
        for row in &v {
            assert_eq!(row.len(), ncols);
            data.extend_from_slice(&row);
        }
        Array2::from_shape_vec((nrows, ncols), data).unwrap()
    }
}
