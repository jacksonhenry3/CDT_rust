use cached::proc_macro::cached;
use std::fs::File;
use std::io::{self, BufWriter, Write};

#[cached]
pub fn choose(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    if k > n {
        return 0;
    }
    match choose(n - 1, k - 1).checked_add(choose(n - 1, k)) {
        Some(x) => x,
        None => panic!("Overflow"),
    }
}

// doesnt seem to matter
#[cached]
fn log_fact(n: usize) -> f64 {
    if n < 2 {
        return 0.0;
    }
    (2..=n).map(|i| (i as f64).ln()).sum()
}

pub fn log_choose(n: usize, k: usize) -> f64 {
    log_fact(n) - log_fact(k) - log_fact(n - k)
}

pub fn write_volume_action_to_csv(data: Vec<(String, f64)>, file_path: &str) -> io::Result<()> {
    let mut csv_data = String::from("");

    // Accumulate all data into the csv_data string
    for (volume_profile, action) in data {
        let row = format!("{},{}\n", volume_profile, action);
        csv_data.push_str(&row);
    }

    // Open the file and create a single write
    let mut file = File::create(file_path)?;
    file.write_all(csv_data.as_bytes())?;

    Ok(())
}
