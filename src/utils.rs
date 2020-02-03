#[allow(dead_code, unused_assignments)]
/// Converts given size in bytes into human readable size.
/// Units are determined automatically:
/// * B
/// * kB (1 decimal place)
/// * MB (1 decimal place)
/// * GB (2 decimal places)
/// Also suports forced decimal places.
pub fn humanize_size(bytes: u64, forced_places: Option<u16>) -> String {
    let mut size = 0.0;
    let mut units = String::new();
    let mut places = 0;
    let fbytes = bytes as f64;

    // Calculate.
    if 1.0 > fbytes / 1000_f64 {
        size = fbytes;
        units = "B".into();
    } else if 1.0 > fbytes / 1000_f64.powf(2.0) {
        size = fbytes / 1000_f64;
        units = "kB".into();
        places = 1;
    } else if 1.0 > fbytes / 1000_f64.powf(3.0) {
        size = fbytes / 1000_f64.powf(2.0);
        units = "MB".into();
        places = 1;
    } else {
        size = fbytes / 1000_f64.powf(3.0);
        units = "GB".into();
        places = 2;
    }

    // Override calculated decimal places.
    if forced_places.is_some() {
        places = forced_places.unwrap();
    }

    // Ensure decimal places.
    let multiplier = 10_f64.powf(places.into());
    size = (size * multiplier).round() / multiplier;

    // Return formatted size.
    format!("{} {}", size, units)
}
