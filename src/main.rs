use clap::{App, Arg};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

use fw_plot::plot::plot;

// read in the output of fasta_windows for kmer spectra
#[derive(Clone, Debug, Deserialize)]
struct Record {
    id: String,
    start: i32,
    end: i32,
    nuc_list: Vec<u32>,
}

// TODO: add dimensions & other colour schemes as options
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
        let path = format!("{}/{}.png", outdir, k);

        plot::plot(v, &path)?;
        eprintln!("[+]\tHeatmap for {}.", k);
        // eprintln!("[+]\tHeatmap for {} at {}", k, path);
    }
    Ok(())
}
