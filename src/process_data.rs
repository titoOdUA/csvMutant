use csv::{Error, StringRecord};

pub fn get_raw_data(origin_rows: &[Result<StringRecord, Error>]) -> Vec<Vec<String>> {
    let mut raw_data_rows = vec![];
    for row in &origin_rows[0..] {
        match row {
            Ok(v) => {
                let row: Vec<String> = v.iter().map(|item| item.to_string()).collect();
                raw_data_rows.push(row);
            }
            Err(_e) => panic!("Can't read row!"),
        };
    }
    raw_data_rows
}

pub fn change_na_to_dashes(raw_data_rows: &mut [Vec<String>]) {
    for row in raw_data_rows.iter_mut() {
        for item in row.iter_mut() {
            if *item == "n/a" {
                *item = String::from("222")
            }
        }
    }
}
