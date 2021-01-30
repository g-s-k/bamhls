use std::{collections::HashMap, error::Error};
use {clap::Clap, m3u8_rs::playlist::VariantStream, reqwest::blocking};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let body = blocking::get(&args.stream_uri)?.text()?;

    match m3u8_rs::parse_master_playlist(body.as_ref()) {
        Ok((_, mut playlist)) => {
            let (mut iframes, variant_groups) = group_variants(playlist.variants.drain(..));
            // organize variant groups alphabetically by codec
            let mut key_sorted_variant_groups = variant_groups.into_iter().collect::<Vec<_>>();
            key_sorted_variant_groups.sort_by_key(|(audio_codec, _)| audio_codec.clone());

            for (_, mut variants) in key_sorted_variant_groups {
                variants.sort_by_key(get_bandwidth);
                // we want descending order - prioritize highest quality and fall back to lower if
                // necessary
                variants.reverse();
                playlist.variants.append(&mut variants);
            }

            playlist.variants.append(&mut iframes);

            if args.debug {
                eprintln!("{:#?}", playlist);
            }

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
    /// Show a verbose debug representation prior to printing output
    #[clap(short('D'), long)]
    debug: bool,
    /// Show a diff between the fetched file and the output
    #[clap(short, long)]
    diff: bool,
    /// The URI to the HLS master playlist you want to inspect
    stream_uri: String,
}

/// Coaelesce variant streams into groups based on audio codec. I-frame variants get special
/// treatment since they are really a different type of entry.
fn group_variants<I: IntoIterator<Item = VariantStream>>(
    vars: I,
) -> (Vec<VariantStream>, HashMap<String, Vec<VariantStream>>) {
    vars.into_iter().fold(
        (Vec::new(), HashMap::new()),
        |(mut iframes, mut groups), var| {
            if var.is_i_frame {
                iframes.push(var);
            } else {
                groups
                    .entry(var.audio.clone().unwrap())
                    .or_insert_with(Vec::new)
                    .push(var);
            }

            (iframes, groups)
        },
    )
}

fn get_bandwidth(var: &VariantStream) -> usize {
    var.bandwidth
        .parse()
        .expect("bandwidth attribute must be a positive integer")
}
