pub mod stat {

    use csv::ReaderBuilder;
    use plotters::prelude::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;
    use std::fs::File;

    // read in the output of fasta_windows for kmer spectra
    // should throw an error if wrong TSV used..?
    #[derive(Clone, Debug, Deserialize)]
    struct Record {
        id: String,
        start: f32,
        end: f32,
        gc_prop: f32,
        gc_skew: f32,
        shannon_entropy: f32,
        prop_gs: f32,
        prop_cs: f32,
        prop_as: f32,
        prop_ts: f32,
        prop_ns: f32,
        dinucleotide_shannon: f32,
        trinucleotide_shannon: f32,
        tetranucleotide_shannon: f32,
    }

    pub fn plot_stat(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        // parse command line options
        let tsv = matches.value_of("tsv").unwrap();
        let variable = matches.value_of("variable").unwrap();
        let outdir = matches.value_of("outdir").unwrap();
        // Read an array back from the file
        let file = File::open(tsv)?;

        // deserialize the tsv into a readerbuilder
        let mut reader = ReaderBuilder::new()
            .flexible(true)
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        // group the tsv into same ID's
        eprintln!("[+]\tMaking BTreeMap of TSV, grouped by ID.");
        let mut groups = BTreeMap::new();

        // skip 1 because we want custom header names
        for result in reader.deserialize().skip(1) {
            let record: Record = result?;
            let key = record.id;
            // only need either start or end...
            let start = record.start;

            let var = match variable {
                "gc_prop" => record.gc_prop,
                "gc_skew" => record.gc_skew,
                "shannon_entropy" => record.shannon_entropy,
                "prop_gs" => record.prop_gs,
                "prop_cs" => record.prop_cs,
                "prop_as" => record.prop_as,
                "prop_ts" => record.prop_ts,
                "prop_ns" => record.prop_ns,
                "dinucleotide_shannon" => record.dinucleotide_shannon,
                "trinucleotide_shannon" => record.trinucleotide_shannon,
                "tetranucleotide_shannon" => record.tetranucleotide_shannon,
                _ => panic!("Variable {} not covered.", variable),
            };

            groups.entry(key).or_insert(Vec::new()).push((start, var));
        }
        for (k, v) in groups {
            let path = format!("{}/{}.png", outdir, k);
            statmap(v, &path, variable)?;
            eprintln!("[+]\tStatmap for {} at {}", k, path);
        }
        Ok(())
    }

    fn statmap(
        data: Vec<(f32, f32)>,
        path: &str,
        variable: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // dimensions of the plot
        let dims = (1280, 2 * 480);

        let max_x = data
            .clone()
            .into_iter()
            .map(|(x, _y)| x)
            .fold(-1. / 0. /* -inf */, f32::max);
        let max_y = data
            .clone()
            .into_iter()
            .map(|(_x, y)| y)
            .fold(-1. / 0. /* -inf */, f32::max);

        let y_min = match variable {
            "gc_skew" => -1f32,
            "shannon_entropy" => 1.5f32,
            "dinucleotide_shannon" => 1.5f32,
            "trinucleotide_shannon" => 1.5f32,
            "tetranucleotide_shannon" => 1.5f32,
            _ => 0f32,
        };

        let root = BitMapBackend::new(path, (dims.0, dims.1)).into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.margin(10, 10, 10, 10);
        // After this point, we should be able to draw construct a chart context
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(40)
            .set_label_area_size(LabelAreaPosition::Left, (8).percent())
            .set_label_area_size(LabelAreaPosition::Bottom, (8).percent())
            // zero first value needs to be replaced by a minimum I think... for the Y
            .build_cartesian_2d(0f32..max_x, y_min..max_y)?;

        let format_variable = match variable {
            "gc_prop" => "GC proportion",
            "gc_skew" => "GC skew",
            "shannon_entropy" => "Shannon entropy",
            "prop_gs" => "Proportion of G's",
            "prop_cs" => "Proportion of C's",
            "prop_as" => "Proportion of A's",
            "prop_ts" => "Proportion of T's",
            "prop_ns" => "Proportion of N's",
            "dinucleotide_shannon" => "Dinucleotide shannon diversity",
            "trinucleotide_shannon" => "Trinucleotide shannon diversity",
            "tetranucleotide_shannon" => "Tetranucleotide shannon diversity",
            _ => variable,
        };

        // Then we can draw a mesh
        chart
            .configure_mesh()
            .y_desc(format_variable)
            .x_desc("Length along genome")
            .x_label_formatter(&|x| match x / 1000000.0 {
                // if greater than megabase, 1 decimal place
                x_ if x_ >= 1.0 => format!("{:.1}Mb", x / 1000000.0),
                // if less than 100kb
                x_ if x_ < 0.1 => format!("{:.0}Kb", x / 1000.0),
                // otherwise, no decimal places
                _ => format!("{:.0}Mb", x / 1000000.0),
            })
            .label_style(TextStyle::from(("sans-serif", 25)))
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()?;

        // And we can draw something in the drawing area
        chart.draw_series(LineSeries::new(data, &BLACK))?;

        Ok(())
    }
}
