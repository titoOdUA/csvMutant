use chrono::NaiveDate;
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

pub fn change_na_values(raw_data_rows: &mut [Vec<String>], new_val: &str) {
    for row in raw_data_rows.iter_mut() {
        for item in row.iter_mut() {
            if *item == "n/a" {
                *item = new_val.to_string();
            }
        }
    }
}

///Change format of date row to the standart one
pub fn change_dates_format(
    raw_data_rows: &mut [Vec<String>],
    original_format: &str,
    dates_row_index: usize,
) {
    let dates_row = raw_data_rows
        .get_mut(dates_row_index)
        .expect("Can't find dates row!");
    println!("Dates row before changes: {:?}", dates_row);
    println!("--------------------------------");
    dates_row.iter_mut().for_each(|value| {
        let naive_date = NaiveDate::parse_from_str(value, original_format).ok();
        if let Some(d) = naive_date {
            *value = d.format("%d.%m.%Y").to_string();
        }
    });

    println!(
        "Dates row after chages: {:?}",
        raw_data_rows.get(dates_row_index)
    );
}
