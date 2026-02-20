use csv;
use std::path::Path;
use std::error::Error;
use crate::model::transaction::Transaction;

/*
 * Parse a CSV file and transform records into transactions
 */
pub fn parse_file_to_transaction(file_path:String) -> Result<Vec<Transaction>, Box<dyn Error>>{

    let mut rcsv = csv::Reader::from_path(Path::new(&file_path))?;

    for result in rcsv.records() {
        let record = result?;
        println!("{:?}", record);
    }
    return Ok(vec![]);

}