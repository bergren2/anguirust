use human_panic::setup_panic;
use serde_json::json;
use structopt::StructOpt;


// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // Output JSON instead of human readable messages
    #[structopt(long = "json")]
    json: bool,
}

fn main() {
    setup_panic!();

    let args = Cli::from_args();

    if args.json {
        println!("{}", json!({
            "type": "message",
            "content": "Hello world",
        }));
    } else {
        println!("Hello world");
    }
}
