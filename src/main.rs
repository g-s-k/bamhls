use std::{collections::HashMap, error::Error};
use {clap::Clap, m3u8_rs::playlist::VariantStream, reqwest::blocking};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let body = blocking::get(&args.stream_uri)?.text()?;

    match m3u8_rs::parse_master_playlist(body.as_ref()) {
        Ok((_, mut playlist)) => {
            // slurp out contents of variant list so we can manipulate them
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

            // also sort and add iframe variants
            iframes.sort_by_key(get_bandwidth);
            iframes.reverse();
            playlist.variants.append(&mut iframes);

            match args.output {
                Output::Debug => println!("{:#?}", playlist),
                Output::Diff => {
                    // write final playlist into an intermediate string so we can compute a diff
                    let mut buffer = Vec::new();
                    playlist.write_to(&mut buffer)?;
                    let stringified_buffer = String::from_utf8(buffer)?;

                    println!("{}", prettydiff::diff_lines(&body, &stringified_buffer));
                }
                Output::M3U8 => {
                    let mut stdout = std::io::stdout();
                    playlist.write_to(&mut stdout)?;
                }
            }
        }

        // ownership rules strike again
        // (could not structure this right to do an idiomatic `return Err(...)` since both variants
        // of the return type include a reference to the local variable `body`)
        Err(e) => panic!("{:#?}", e),
    }

    Ok(())
}

/// A utility to fetch and sort m3u8 master playlists for HLS
#[derive(Clap, Debug)]
#[clap(author = "George Kaplan <george@georgekaplan.xyz>")]
struct Args {
    /// Select output format
    #[clap(short, long, arg_enum, default_value = "m3u8")]
    output: Output,
    /// The URI to the HLS master playlist you want to inspect
    stream_uri: String,
}

#[derive(Clap, Debug, PartialEq)]
enum Output {
    M3U8,
    Diff,
    Debug,
    // We can add more formats here in the future...
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

/// Extract bandwidth attribute from EXT-X-STREAM-INF variants
fn get_bandwidth(var: &VariantStream) -> usize {
    var.bandwidth
        .parse()
        .expect("bandwidth attribute must be a positive integer")
}
