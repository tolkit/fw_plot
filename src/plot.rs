pub mod plot {
    use colorous;
    use ndarray::Array2;
    use ndarray_stats;
    use ndarray_stats::QuantileExt;
    use plotters::prelude::*;

    pub fn plot_heatmap(
        array: &Array2<u32>,
        image_dimensions: (u32, u32),
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // create the root
        let root = BitMapBackend::new(
            path,             // path
            image_dimensions, // width x height
        )
        .into_drawing_area();

        // so that in our graph, x = time, y = frequency.
        let (num_samples, num_freq_bins) = match array.shape() {
            &[num_rows, num_columns] => (num_rows, num_columns),
            _ => panic!("Heatmap is a {}D array, expected a 2D array. This should never happen (should not be possible to call function with anything but a 2d array)", 
                array.ndim()
            )
        };

        let heatmap_cells = root.split_evenly((num_freq_bins, num_samples));

        // Scaling values
        let windows_scaled = array.map(|i| *i as f32 / (num_freq_bins as f32));
        let highest_heatmap_density: &f32 = windows_scaled.max_skipnan();
        // Finally add a color scale
        let color_scale = colorous::TURBO;

        for (cell, heatmap_density) in heatmap_cells.iter().zip(windows_scaled.iter()) {
            let heatmap_density_scaled = heatmap_density.sqrt() / highest_heatmap_density.sqrt();
            let color = color_scale.eval_continuous(heatmap_density_scaled as f64);
            cell.fill(&RGBColor(color.r, color.g, color.b)).unwrap();
        }

        Ok(())
    }
}
