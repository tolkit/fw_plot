use clap::{App, Arg};
use std::error::Error;
use std::process;

use fw_plot::heatmap::heatmap;
use fw_plot::stat::stat;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("fw_plot")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Create fast and simple heatmaps of fasta_windows output.")
        .subcommand(
            clap::SubCommand::with_name("heatmap")
                .about("Make a heatmap of the kmer frequencies across chromosomes.")
                .arg(
                    Arg::with_name("tsv")
                        .short("t")
                        .long("tsv")
                        .takes_value(true)
                        .required(true)
                        .help("The TSV file (..._di/tri/tetranuc_windows.tsv)."),
                )
                .arg(
                    Arg::with_name("colour")
                        .short("c")
                        .long("colour")
                        .takes_value(true)
                        .required(true)
                        .default_value("TURBO")
                        // there are more... if there's appetite
                        .possible_values(&[
                            "TURBO",
                            "VIRIDIS",
                            "INFERNO",
                            "MAGMA",
                            "PLASMA",
                            "CIVIDIS",
                            "WARM",
                            "COOL",
                            "CUBEHELIX",
                        ])
                        .help("The colour scale that the heatmap uses. See https://docs.rs/colorous/1.0.5/colorous/."),
                )
                .arg(
                    Arg::with_name("outdir")
                        .short("o")
                        .long("outdir")
                        .takes_value(true)
                        .required(true)
                        .default_value(".")
                        .help("The output directory."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("stat")
                .about("Quickly plot fundamental sequence statistics across chromosomes.")
                .arg(
                    Arg::with_name("tsv")
                        .short("t")
                        .long("tsv")
                        .takes_value(true)
                        .required(true)
                        .help("The TSV file (..._windows.tsv)."),
                )
                .arg(
                    Arg::with_name("variable")
                        .short("v")
                        .long("variable")
                        .takes_value(true)
                        .required(true)
                        .possible_values(&[
                            "gc_prop",
                            "gc_skew",
                            "shannon_entropy",
                            "prop_gs",
                            "prop_cs",
                            "prop_as",
                            "prop_ts",
                            "prop_ns",
                            "dinucleotide_shannon",
                            "trinucleotide_shannon",
                            "tetranucleotide_shannon",
                        ])
                        .help("The variable to plot."),
                )
                .arg(
                    Arg::with_name("outdir")
                        .short("o")
                        .long("outdir")
                        .takes_value(true)
                        .required(true)
                        .default_value(".")
                        .help("The output directory."),
                ),
        )
        .get_matches();

    let subcommand = matches.subcommand();
    match subcommand.0 {
        "heatmap" => {
            let matches = subcommand.1.unwrap();
            heatmap::plot_heatmap(matches)?;
        }
        "stat" => {
            let matches = subcommand.1.unwrap();
            stat::plot_stat(matches)?;
        }
        _ => {
            println!(
                "Subcommand invalid, run with '--help' or '-h' for subcommand options. Exiting."
            );
            process::exit(1);
        }
    }

    Ok(())
}
