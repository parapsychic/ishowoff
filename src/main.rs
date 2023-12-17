use std::{time::{self, SystemTime, Duration}, fs, io::Write};

use ishowoff_github::{make_svg, make_theme, parse_args, parse_time};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && &args[1] == "help" {
        usage();
        std::process::exit(0);
    }

    let map = match parse_args(args) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{}", x);
            usage();
            std::process::exit(0);
        }
    };

    // parse the timestamp and put it in
    let session_time = if let Some(x) = map.get("session-text-time"){
        x.clone()
    }
    else{

    let start_time = if let Some(x) = map.get("starttime") {
        match x.parse::<u64>() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Start time should be in unix epochs");
                std::process::exit(0);
            }
        }
    } else {
        eprintln!("Start time not specified");
        usage();
        std::process::exit(0);
    };
        parse_time(start_time)
        
    };

    let svg = make_svg(make_theme(map), &session_time);
    match write_file("stats.svg", &svg) {
        Ok(_) => println!("File written successfully!"),
        Err(e) => println!("Error writing file: {}", e),
    }
}

fn usage() {
    println!(
        "USAGE: 
        For defaults, supply only the timestamp: ishowoff starttime=10:00
        \tTheme:                    theme=[dark,light]
        \tBackground color:         bg-primary
        \tBackground stroke:        bg-stroke
        \tLanguage:                 lang
        \tWorkspace text color:     primary-color
        \tWorkspace text font:      primary-font
        \tWorkspace text:           primary-text
        \tTimestamp color:          session-color
        \tSession text font:        session-font
        \tExplicitly set timestamp: session-text-time
        \tEditor text color:        editor-color
        \tEditor text font:         editor-font
        \tEditor text:              editor-text"
    );
}

fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}