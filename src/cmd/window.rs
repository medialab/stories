use chrono_tz::Tz;
use clap::Parser;
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;

use crate::cli_utils::{acquire_progress_indicator, get_column_index};
use crate::date_utils::inferred_date;

#[derive(Parser, Debug)]
#[clap(
    about = "Infer the size, in number of documents, of the time window for the clustering algorithm."
)]
pub struct Opts {
    input: String,

    ///Use --raw to get the number of documents without other message.
    #[clap(long)]
    raw: bool,

    ///Name of the column in the csv input containing the dates of the documents
    #[clap(long, default_value = "created_at")]
    datecol: String,

    ///Size of the window in number of days
    #[clap(long, value_parser, default_value_t = 0.5)]
    // default_value works on pre-parsed values (ie strings), like #[clap(default_value = "100")]
    // default_value_t works on post-parsed values (ie your data type), like #[clap(default_value_t = 100)]
    size: f64,

    #[clap(long)]
    total: Option<u64>,

    #[clap(long)]
    tsv: bool,

    ///If timestamps are provided, they will be parsed as UTC timestamps, then converted to
    ///the provided timezone. Other date formats will not be converted.
    #[clap(long, default_value = "Europe/Paris")]
    timezone: String,
}

// TODO: could infer window from the vocab command in fact
pub fn run(cli_args: &Opts) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if cli_args.tsv { b'\t' } else { b',' })
        .from_path(&cli_args.input)?;

    let bar = acquire_progress_indicator("Processing tweets", cli_args.total);

    let headers = rdr.headers()?;

    let date_column_index = get_column_index(&headers, &cli_args.datecol)?;

    let mut days: HashMap<String, usize> = HashMap::new();

    for result in rdr.records() {
        bar.inc(1);

        let record = result?;

        let date_cell = &record[date_column_index];

        let tz: Tz = cli_args.timezone.parse().or(Err("Unknown timezone"))?;

        let local_datetime = inferred_date(&date_cell, &tz)?;

        let day = local_datetime.format("%Y-%m-%d").to_string();

        days.entry(day).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut day_average = 0;

    for day_count in days.values() {
        day_average += day_count;
    }

    let window = (cli_args.size * (day_average as f64) / (days.len() as f64)).floor() as usize;

    bar.finish_at_current_pos();

    if cli_args.raw {
        println!("{:?}", window);
    } else {
        eprintln!("Optimal window size is: {:?}", window);
    }
    Ok(())
}
