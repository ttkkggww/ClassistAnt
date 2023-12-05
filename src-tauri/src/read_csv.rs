// Cargo.tomlに依存関係を追加する: csv = "1.1"
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;


#[tauri::command]
pub async fn read_csv_file(file_path: &str) -> Result<(),String> {
    let file = File::open(file_path).unwrap();
    let mut rdr = ReaderBuilder::new().from_reader(file);

    for result in rdr.records() {
        let record = result.unwrap();
        // recordを使ってデータを処理する
        println!("{:?}", record);
    }

    Ok(())
}
