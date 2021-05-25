use clap::{App, Arg};
use std::error::Error;
use std::process;

use fw_plot::heatmap::heatmap;
use fw_plot::stat::stat;

// TODO: add dimensions & other colour schemes as options
//     : output window size?
//     : optional kmers on yaxis?
//     : AA at bottom, TT at top of yaxis

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("fw_plot")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Create fast and simple heatmaps of fasta_windows output.")
        .subcommand(
            clap::SubCommand::with_name("heatmap")
                .about("Supply a di/tri/tetranucleotide TSV")
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
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("stat")
                .about("Supply the main TSV, extract and plot a variable across chromosomes.")
                .arg(
                    Arg::with_name("tsv")
                        .short("t")
                        .long("tsv")
                        .takes_value(true)
                        .required(true)
                        .help("The TSV file."),
                )
                .arg(
                    Arg::with_name("variable")
                        .short("v")
                        .long("variable")
                        .takes_value(true)
                        .required(true)
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
