use arrow::csv::{Writer, WriterBuilder};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let file = File::create("output/data.csv").unwrap();
    let builder = WriterBuilder::new().has_headers(false);
    let mut writer = builder.build(file);

    read_parquet_dir(Path::new("./data/"), &mut writer)?;

    Ok(())
}

fn read_parquet_dir(dir_path: &Path, writer: &mut Writer<File>) -> anyhow::Result<()> {
    let dir = std::fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        if entry.path().is_file() {
            if entry.path().extension().unwrap() == "parquet" {
                read_parquet_file(&entry.path(), writer)?;
            }
        }
    }

    Ok(())
}

fn read_parquet_file(file_path: &Path, writer: &mut Writer<File>) -> anyhow::Result<()> {
    let file = File::open(file_path).unwrap();

    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;

    let mut row_num = 0;
    for batch in reader {
        let record_batch = batch?;

        writer.write(&record_batch)?;

        row_num += record_batch.num_rows();
    }

    println!("read {row_num} rows from {file_path:?}");

    Ok(())
}
