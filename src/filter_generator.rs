use std::{ffi::OsString, fmt::Display};

enum Size {
    GB(f64),
    MB(f64),
}

enum FilterSize {
    Small,
    Medium,
    Large,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            Size::GB(num) => format!("{:.1} GiB", num),
            Size::MB(num) => format!("{:.1} MiB", num),
        };
        write!(f, "{output}")
    }
}

const SIZE: f64 = 847_223_402.0;
const SMALL_FILTER_SIZE: Size = Size::MB(SIZE * 9.1 / 8.0 / 1024.0 / 1024.0);
const MEDIUM_FILTER_SIZE: Size = Size::GB(SIZE * 18.1 / 8.0 / 1024.0 / 1024.0 / 1024.0);
const LARGE_FILTER_SIZE: Size = Size::GB(SIZE * 36.2 / 8.0 / 1024.0 / 1024.0 / 1024.0);

pub fn generate_filter(input: OsString, output: OsString) {
    println!("There are 3 sizes of filters: Small (s), Medium (m), and Large (l)");
    println!("The filter can find compromised password with 100% accuracy, but it may show passwords that are not compromised as compromised (false positives)");
    println!(
        "The small filter has false positivity rate of 0.4% (1 in 256) and needs {}",
        SMALL_FILTER_SIZE
    );
    println!(
        "The medium filter has false positivity rate of 0.016% (1 in 65,536) and needs {}",
        MEDIUM_FILTER_SIZE
    );
    println!(
        "The large filter has false positivity rate of 1 in 4,294,967,296 and needs {}",
        LARGE_FILTER_SIZE
    );

    println!("Please choose a size (s, m, l):");
    let result;
    loop {
        let mut filter_size = String::new();
        let input = std::io::stdin().read_line(&mut filter_size);
        match input {
            Ok(_) => {
                result = match filter_size.lines().next().unwrap() {
                    "s" => FilterSize::Small,
                    "m" => FilterSize::Medium,
                    "l" => FilterSize::Large,
                    other => {
                        println!("Wrong option. Try again:");
                        continue;
                    }
                };
                break;
            }
            Err(_) => continue,
        }
    }

    todo!();
}
