pub mod plot {

    use plotters::prelude::*;

    pub fn plot(data: Vec<Vec<u32>>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // dimensions of the plot
        let dims = (1280, 2 * 480);
        // colour scale
        let color_scale = colorous::TURBO;

        // the root of the plot - bitmap is cheap to compute
        let root = BitMapBackend::new(path, (dims.0, dims.1)).into_drawing_area();
        root.fill(&WHITE)?;

        // legend must be drawn here, before
        let leg_props = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

        // draw legend and the proportions
        for prop in leg_props {
            let color = color_scale.eval_continuous(prop);
            root.draw(&Rectangle::new(
                [
                    (
                        (dims.0 as f64 * 0.95) as i32,
                        ((dims.1 as f64 * (prop * 0.8)) + 45f64) as i32,
                    ),
                    (
                        (dims.0 as f64 * 0.97) as i32,
                        ((dims.1 as f64 * ((prop * 0.8) + 0.1)) + 45f64) as i32,
                    ),
                ],
                RGBColor(color.r, color.g, color.b).filled(),
            ))?;
            // add text
            root.draw(&Text::new(
                format!("{}", prop),
                (
                    (dims.0 as f64 * 0.98) as i32,
                    ((dims.1 as f64 * (prop * 0.8)) + 80f64) as i32,
                ),
                ("sans-serif", 15).into_font(),
            ))?;
        }

        // add a margin
        let root = root.margin(10, 10, 10, 100);

        // the number of windows & kmers
        let x_len = data.len() as f32;
        // messy
        let y_len = data.clone().into_iter().nth(0).unwrap().len() as f32;

        let mut chart = ChartBuilder::on(&root).build_cartesian_2d(0f32..x_len, 0f32..y_len)?;

        chart.configure_mesh().disable_mesh().draw()?;

        let plotting_area = chart.plotting_area();

        // to get the max value for colour scale
        fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
            nested.into_iter().flatten().collect()
        }

        let highest_frequency = *flatten(data.clone()).iter().max().unwrap();

        let mut top_left = 0.0;
        let bottom_right = 1.0;

        for (window, data_per_window) in data.into_iter().enumerate() {
            for (kmer, frequency) in data_per_window.into_iter().enumerate() {
                // otherwise the last kmer width is 1 pixel for some reason.
                let kmer = kmer + 1;

                let frequency_scaled =
                    (frequency as f64).sqrt() / (highest_frequency as f64).sqrt();
                let color = color_scale.eval_continuous(frequency_scaled);

                top_left = 1.0;
                // ew all those f32's
                if kmer == y_len as usize {
                    top_left = 0.0;
                }

                let rect = Rectangle::new(
                    [
                        (window as f32, kmer as f32 + top_left as f32),
                        (
                            window as f32 + 1.0,
                            (kmer as f32 - bottom_right as f32).abs(),
                        ),
                    ],
                    RGBColor(color.r, color.g, color.b).filled(),
                );
                plotting_area.draw(&rect)?;
            }
        }

        Ok(())
    }
}
