use std::boxed::Box;
use std::error::Error;
use std::sync::Mutex;

use clap::Clap;
use rayon::prelude::*;

use crate::cli_utils::{acquire_progress_indicator, acquire_tokenizer, ReorderedWriter};

#[derive(Clap, Debug)]
#[clap(about = "Tokenize tweet text contained in a CSV file.")]
pub struct Opts {
    column: String,
    input: String,
    #[clap(long)]
    total: Option<u64>,
    #[clap(long)]
    tsv: bool,
}

pub fn run(cli_args: &Opts) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if cli_args.tsv { b'\t' } else { b',' })
        .from_path(&cli_args.input)?;

    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&csv::StringRecord::from(vec!["tokens"]))?;

    let bar = acquire_progress_indicator("Tokenizing tweets", cli_args.total);

    let headers = rdr.headers()?;

    let column_index = headers.iter().position(|v| v == cli_args.column);

    if column_index.is_none() {
        return Err(format!(
            "Column \"{}\" does not exist in given CSV file!",
            cli_args.column
        )
        .into());
    }

    let column_index = column_index.unwrap();

    let tokenizer = acquire_tokenizer();
    let reordered_writer = ReorderedWriter::new(&mut wtr);
    let mutex = Mutex::new(reordered_writer);

    rdr.records()
        .enumerate()
        .par_bridge()
        .map(|(i, result)| {
            let record = result.expect("Could not read row!");

            (
                i,
                tokenizer
                    .tokenize(
                        &record
                            .get(column_index)
                            .expect("Found a row with fewer columns than expected!"),
                    )
                    .collect::<Vec<String>>(),
            )
        })
        .for_each(|(i, tokens)| {
            bar.inc(1);

            let mut locked_wtr = mutex.lock().unwrap();

            locked_wtr.write_vec(i, vec![tokens.join("|")]);
        });

    bar.finish_at_current_pos();

    Ok(())
}