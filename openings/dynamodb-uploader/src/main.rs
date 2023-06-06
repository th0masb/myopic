#[macro_use]
extern crate serde_derive;

use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, BatchWriteItemInput, DynamoDb, DynamoDbClient, PutRequest, WriteRequest,
};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;
use tokio::time::Duration;

const PAUSE_DURATION_MILLIS: u64 = 1000;

#[derive(Debug, StructOpt, Serialize)]
#[structopt(name = "uploader")]
struct Opt {
    /// The source file containing the table entries to write.
    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,
    /// The name of the dynamodb table to write to.
    #[structopt(short, long)]
    table: String,
    /// The AWS region in which the target table lives.
    #[structopt(short, long, parse(try_from_str))]
    region: Region,
    /// Write capacity units provisioned for the table.
    #[structopt(short, long)]
    wcu: usize,
    /// In the table moves and frequencies will be combined as a
    /// single string, this allows you to customise the separator
    /// string.
    #[structopt(name = "frequency-separator", long, default_value = ":")]
    #[serde(rename = "frequency-separator")]
    frequency_separator: String,
    /// The primary key column in the table representing the board
    /// position.
    #[structopt(name = "position-attribute", long, default_value = "PositionFEN")]
    #[serde(rename = "position-attribute")]
    position_attribute: String,
    /// The attribute in the table representing the suggested moves.
    #[structopt(name = "moves-attribute", long, default_value = "Moves")]
    #[serde(rename = "moves-attribute")]
    moves_attribute: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    eprintln!("{}", chrono::Utc::now());
    eprintln!("Starting DynamoDB uploader with parameters:");
    eprintln!("{}", serde_json::to_string_pretty(&opt)?);
    eprintln!("...");
    let mut entries = load_write_requests(&opt)?;
    let total_writes = entries.len();
    let rt = tokio::runtime::Runtime::new()?;
    let client = DynamoDbClient::new(opt.region.clone());

    let progress = indicatif::ProgressBar::new(total_writes as u64);
    progress.set_message("Upload progress");
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}")?
            .progress_chars("##-"),
    );
    while entries.len() > 0 {
        let mut request_items = HashMap::new();
        request_items.insert(opt.table.clone(), draw_n_entries(&mut entries, opt.wcu));

        rt.block_on(client.batch_write_item(BatchWriteItemInput {
            request_items,
            return_consumed_capacity: None,
            return_item_collection_metrics: None,
        }))?
        .unprocessed_items
        .into_iter()
        .for_each(|mut unprocessed_map| {
            match unprocessed_map.remove(&opt.table) {
                None => {}
                // If items were not processed put them back in the processing list
                Some(mut items) => entries.append(&mut items),
            }
        });

        progress.set_position((total_writes - entries.len()) as u64);
        // Need to sleep before next upload as table writes are throttled
        thread::sleep(Duration::from_millis(PAUSE_DURATION_MILLIS));
    }

    progress.finish();

    Ok(())
}

fn draw_n_entries<T>(source: &mut Vec<T>, n: usize) -> Vec<T> {
    let mut dest = Vec::with_capacity(n);
    for _ in 0..n {
        match source.pop() {
            None => break,
            Some(entry) => dest.push(entry),
        }
    }
    dest
}

fn load_write_requests(options: &Opt) -> Result<Vec<WriteRequest>, Box<dyn Error>> {
    Ok(BufReader::new(File::open(&options.source)?)
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|entry| serde_json::from_str::<SourceEntry>(&entry).ok())
        .map(|entry| entry.to_write_request(options))
        .collect())
}

#[derive(Clone, Deserialize)]
struct MoveRecord {
    mv: String,
    freq: usize,
}

#[derive(Deserialize)]
struct SourceEntry {
    position: String,
    moves: Vec<MoveRecord>,
}

impl SourceEntry {
    fn to_write_request(self, options: &Opt) -> WriteRequest {
        let mut position_attribute_value = AttributeValue::default();
        position_attribute_value.s = Some(self.position);
        let mut moves_attribute_value = AttributeValue::default();
        moves_attribute_value.ss = Some(
            self.moves
                .iter()
                .map(|record| {
                    format!("{}{}{}", record.mv, &options.frequency_separator, record.freq)
                })
                .collect(),
        );

        let mut item = HashMap::new();
        item.insert(options.position_attribute.clone().to_string(), position_attribute_value);
        item.insert(options.moves_attribute.clone(), moves_attribute_value);

        WriteRequest { delete_request: None, put_request: Some(PutRequest { item }) }
    }
}
