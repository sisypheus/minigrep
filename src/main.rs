use std::{env, error::Error, fs, process};

struct Args {
    hay: String,
    needle: String,
}

impl Args {
    fn new(args: Vec<String>) -> Result<Args, Box<dyn Error>> {
        match args[..] {
            [_, ref needle, ref hay] => Ok(Args {
                hay: hay.to_string(),
                needle: needle.to_string(),
            }),
            _ => Err("Usage: needle haystack".into()),
        }
    }
}

fn open_and_read(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_name)?;

    let lines: Vec<String> = content.lines().map(String::from).collect();
    Ok(lines)
}

fn search(args: &Args, lines: &Vec<String>) -> String {
    let mut res = String::from("");
    for (idx, line) in lines.iter().enumerate() {
        if line.contains(&args.needle) {
            res.push_str(
                format!(
                    "{}:{}\n",
                    idx + 1,
                    line.replace(
                        &args.needle,
                        format!("\x1b[91m{}\x1b[0m", &args.needle).as_str()
                    )
                )
                .as_str(),
            );
        }
    }
    res
}

fn display_result(mut result: String) {
    if !result.is_empty() {
        result.pop();
    }
    println!("{result}")
}

fn main() {
    let args = Args::new(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let content = match open_and_read(&args.hay) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Problem with the file: {} {}", &args.hay, e);
            process::exit(1);
        }
    };

    let result = search(&args, &content);

    display_result(result)
}
