// Copyright 2019-2020, Markus Näther <naether.markus@gmail.com>

extern crate clap;
extern crate reqwest;
use clap::{Arg, App};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::Duration;
use std::thread;

static APP_USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"/",
	env!("CARGO_PKG_VERSION"),
);

#[tokio::main]
async fn main()  -> Result<(), reqwest::Error> {
	// Argparse
	let arguments = App::new("ArXiV Fetcher")
		.version("1.0")
		.author("Markus Näther <naether.markus@gmail.com>")
		.arg(Arg::with_name("start_year")
			.short("y")
			.long("start_year")
			.help("The start year")
			.takes_value(true))
		.arg(Arg::with_name("start_month")
			.short("m")
			.long("start_month")
			.help("The start month")
			.takes_value(true))
		.get_matches();

	// Relevant arguments
	let year: u32 = arguments.value_of("start_year").unwrap_or("08").parse().unwrap();
    println!("Value for start_year: {}", year);
	let month: u32 = arguments.value_of("start_month").unwrap_or("01").parse().unwrap();
	println!("Value for start_month: {}", month);

	let mut step: u32 = 1000;

	let output_path = Path::new("paper_ids.txt");
	let display = output_path.display();

	let mut output_file = match File::create(&output_path) {
			Err(why) => panic!("couldn't create {}: {}", display, why),
			Ok(output_file) => output_file,
	};

	let client = reqwest::Client::builder()
		.user_agent(APP_USER_AGENT)
		.timeout(Duration::from_secs(60))
		.build()?;
	let milli_sleep = Duration::from_millis(100);
	for y in year..21 {
		for m in 1u32..13 {
			let mut escaper: bool = false;
			let mut p:u32 = 1;
			step = 1000;
			while p < 99999 {
				let _url_match = format!("https://export.arxiv.org/abs/{:02}{:02}.{:05}", y, m, p);
									
				let res = client.get(&_url_match).send().await?;

				if res.status().as_str() == "200" {	
					thread::sleep(milli_sleep);
					println!("?? Checked element at position: {}\n", p);					
					p += step;
				} else {

					if step == 1 {
						println!("!! Found last element at position: {}\n", p);
						escaper = true;
					} else {
						println!("?? Element at position invalid: {}\n", p);
						p -= step;
						step /= 2;
						if step == 0 {
							step = 1;
						}
						//p += step;
						println!("??!! Decreased stepping to: {}\n", step);
					}
				}

				if escaper == true {
					break;
				}
			}
			if p >= 99999 {
				println!("!! WARN: Very high p in y={}, m={}, p={}", y, m, p);
				p = 99999;
			}
			// Write everything to file
			for o in 1u32..p {
				output_file.write_all(format!("https://export.arxiv.org/e-print/{:02}{:02}.{:05}\n", y, m, o).as_bytes());
			}
		}
	}

	Ok(())
}
