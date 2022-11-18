use shellwords;
use std::collections::{self, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(file_name: String) -> Result<String, Box<dyn Error>> {
    // Create a path to the desired file
    let path = Path::new(&file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => Err(format!("couldn't open {}: {}", display, why))?,
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    Ok(file_contents)
}

pub fn parse_curl() -> HashMap<String, String> {
    let file_contents = read_file("src/curl_request.sh".to_string()).unwrap_or_default();
    let commands = shellwords::split(&file_contents[..]).unwrap();
    let args = commands
        .iter()
        .filter(|argument| argument.contains("cookie"))
        .collect::<Vec<&String>>();

    let cookie = args.first().unwrap().splitn(2, ":").last().unwrap().trim();
    let cookie_vector = cookie
        .split(";")
        .map(|ele| {
            let res = ele.split("=").map(|ele| ele.trim()).collect::<Vec<&str>>();
            let tuple = (res[0], res[1]);
            tuple
        })
        .collect::<Vec<(&str, &str)>>();

    //FIXME: If possible avoid the clones
    let hash_map: HashMap<String, String> = cookie_vector
        .into_iter()
        .map(|ele| (ele.0.to_owned(), ele.1.to_owned()))
        .collect();

    hash_map
}
