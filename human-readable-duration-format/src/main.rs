#[derive(Debug)]
enum TimeUnit {
    YEAR,
    DAY,
    HOUR,
    MINUTE,
    SECOND,
}

#[derive(Debug)]
struct Time {
    unit: TimeUnit,
    scalar: u64,
}

impl Time {
    fn format_to_english(&self) -> String {
        match self.unit {
            TimeUnit::YEAR => match self.scalar {
                1 => format!("{} year", 1),
                n => format!("{} years", n),
            },
            TimeUnit::DAY => match self.scalar {
                1 => format!("{} day", 1),
                n => format!("{} days", n),
            },
            TimeUnit::HOUR => match self.scalar {
                1 => format!("{} hour", 1),
                n => format!("{} hours", n),
            },
            TimeUnit::MINUTE => match self.scalar {
                1 => format!("{} minute", 1),
                n => format!("{} minutes", n),
            },
            TimeUnit::SECOND => match self.scalar {
                1 => format!("{} second", 1),
                n => format!("{} seconds", n),
            },
        }
    }
}

fn decompose(seconds: u64, acc: Option<Vec<Time>>) -> Vec<Time> {
    let mut accumulated = match acc {
        Some(vec) => vec,
        None => vec![],
    };

    match seconds {
        0 => accumulated,
        31536000..=std::u64::MAX => {
            accumulated.push(Time {
                scalar: seconds / 31536000,
                unit: TimeUnit::YEAR,
            });
            decompose(seconds % 31536000, Some(accumulated))
        }
        86400..=31535999 => {
            accumulated.push(Time {
                scalar: seconds / 86400,
                unit: TimeUnit::DAY,
            });
            decompose(seconds % 86400, Some(accumulated))
        }
        3600..=86399 => {
            accumulated.push(Time {
                scalar: seconds / 3600,
                unit: TimeUnit::HOUR,
            });
            decompose(seconds % 3600, Some(accumulated))
        }
        60..=3599 => {
            accumulated.push(Time {
                scalar: seconds / 60,
                unit: TimeUnit::MINUTE,
            });
            decompose(seconds % 60, Some(accumulated))
        }
        1..=59 => {
            accumulated.push(Time {
                scalar: seconds,
                unit: TimeUnit::SECOND,
            });
            decompose(0, Some(accumulated))
        }
    }
}

fn format_duration(seconds: u64) -> String {
    let mut decomposed = decompose(seconds, None);
    let last = decomposed.pop();
    let rest = decomposed
        .iter()
        .map(|curr| format!("{}", &curr.format_to_english()))
        .collect::<Vec<String>>()
        .join(", ");

    match last {
        Some(time) => {
            if rest.is_empty() {
                return format!("{}", time.format_to_english());
            }

            return format!("{} and {}", rest, time.format_to_english());
        }
        None => "now".to_string(),
    }
}

fn main() {
    println!("Hello World!");
    println!("{} = {}", format_duration(1), "1 second");
    println!("{} = {}", format_duration(62), "1 minute and 2 seconds");
    println!("{} = {}", format_duration(120), "2 minutes");
    println!("{} = {}", format_duration(3600), "1 hour");
    println!(
        "{} = {}",
        format_duration(3662),
        "1 hour, 1 minute and 2 seconds"
    );
}

#[test]
fn test_basic() {
    assert_eq!(format_duration(1), "1 second");
    assert_eq!(format_duration(62), "1 minute and 2 seconds");
    assert_eq!(format_duration(120), "2 minutes");
    assert_eq!(format_duration(3600), "1 hour");
    assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
}
