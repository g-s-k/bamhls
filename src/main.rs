use std::error::Error;
use {clap::Clap, reqwest::blocking};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let body = blocking::get(&args.stream_uri)?.text()?;

    match m3u8_rs::parse_master_playlist(body.as_ref()) {
        Ok((_, playlist)) => {
            if args.diff {
                // write final playlist into an intermediate string so we can compute a diff
                let mut buffer = Vec::new();
                playlist.write_to(&mut buffer)?;
                let stringified_buffer = String::from_utf8(buffer)?;

                println!("{}", prettydiff::diff_lines(&body, &stringified_buffer));
            } else {
                let mut stdout = std::io::stdout();
                playlist.write_to(&mut stdout)?;
            }
        }

        // ownership rules strike again
        // (could not structure this right to do an idiomatic `return Err(...)` since both variants
        // of the return type include a reference to the local variable `body`)
        Err(e) => panic!("{:#?}", e),
    }

    Ok(())
}

#[derive(Clap, Debug)]
struct Args {
    /// Show a diff between the fetched file and the output
    #[clap(short, long)]
    diff: bool,
    /// The URI to the HLS master playlist you want to inspect
    stream_uri: String,
}
