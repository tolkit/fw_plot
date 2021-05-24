use clap::{App, Arg};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

use fw_plot::plot::plot;
use fw_plot::vec2arr::vec2arr;

// read in the output of fasta_windows for kmer spectra
#[derive(Clone, Debug, Deserialize)]
struct Record {
    id: String,
    start: i32,
    end: i32,
    nuc_list: Vec<u32>,
}

// TODO: add dimesions & other colour schemes as options
//     : output window size?
//     : optional kmers on yaxis?

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("fw_plot")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Create fast and simple heatmaps of fasta_windows output.")
        .arg(
            Arg::with_name("tsv")
                .short("t")
                .long("tsv")
                .takes_value(true)
                .required(true)
                .help("The TSV file."),
        )
        .arg(
            Arg::with_name("outdir")
                .short("o")
                .long("outdir")
                .takes_value(true)
                .required(true)
                .default_value(".")
                .help("The output directory."),
        )
        .get_matches();
    // parse command line options
    let tsv = matches.value_of("tsv").unwrap();
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
    for result in reader.deserialize() {
        let record: Record = result?;
        let key = record.id;
        let value = record.nuc_list;
        groups.entry(key).or_insert(Vec::new()).push(value);
    }

    for (k, v) in groups {
        // key is the ID, value is the Vec<Vecu32>>
        let v_array2 = vec2arr::vec_to_array(v);

        let image_height = 856;
        let image_width = 1024;
        let image_dimensions: (u32, u32) = (image_width as u32, image_height as u32);

        let path = format!("{}/{}.png", outdir, k);

        plot::plot_heatmap(&v_array2, image_dimensions, &path)?;

        eprintln!("[+]\tHeatmap for {} at {}", k, path);
    }
    Ok(())
}
