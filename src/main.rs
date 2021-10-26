use argh::FromArgs;
use libflate::gzip;
use rust_sbml::Model;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(FromArgs)]
/// Print a human friendly summary of the Reactions and Metabolites of a SBML
struct Args {
    #[argh(positional)]
    filename: String,
    #[argh(
        description = "whether to output color (auto | ansi | always)",
        option,
        default = "String::from(\"auto\")"
    )]
    color: String,
}

/// Bump the contents of filename into buffer, may decompressing the file if gz
fn read_to_string_maybe_gz(buf: &mut String, filename: &Path) -> std::io::Result<()> {
    let file = std::fs::File::open(filename).unwrap();
    if filename.ends_with("gz") {
        let mut decoder = gzip::Decoder::new(file)?;
        decoder.read_to_string(buf)?;
    } else {
        let mut bf = BufReader::new(file);
        bf.read_to_string(buf)?;
    }
    Ok(())
}

/// Transform from a str input (coming from CLI) to a ColorChoice enum
fn get_color_choice(color: &str) -> ColorChoice {
    match color {
        "always" => ColorChoice::Always,
        "ansi" => ColorChoice::AlwaysAnsi,
        "auto" => {
            if atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        _ => ColorChoice::Never,
    }
}

fn main() {
    let args: Args = argh::from_env();
    // let file_str = std::fs::read_to_string(&args.filename).unwrap();
    let mut file_str = String::new();
    let path = std::path::Path::new(&args.filename);
    match read_to_string_maybe_gz(&mut file_str, path) {
        Ok(_) => {}
        Err(e) => println!("File could not be read: {}", e),
    }
    let color = get_color_choice(args.color.as_str());
    let mut stdout = StandardStream::stdout(color);
    match Model::parse(&file_str) {
        Ok(mut model) => {
            // Each reaction is formatted to (with colors):
            //     (Reaction) id: [reactants] -> [products] (name)
            let params = &model.parameters;
            model.reactions.iter_mut().for_each(|(id, reac)| {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(243))))
                    .unwrap();
                write!(&mut stdout, "(Reaction)").unwrap();
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                write!(&mut stdout, " {}", id).unwrap();
                stdout.reset().unwrap();
                write!(
                    &mut stdout,
                    ": {} -> {} ({}<{})",
                    reac.list_of_reactants
                        .species_references
                        .iter_mut()
                        .map(|sp| {
                            let species = std::mem::take(&mut sp.species);
                            match sp.stoichiometry {
                                None => species,
                                Some(x) => match x as u8 {
                                    1 => species,
                                    _ => format!("{} {}", x, species),
                                },
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" + "),
                    reac.list_of_products
                        .species_references
                        .iter_mut()
                        .map(|sp| {
                            let species = std::mem::take(&mut sp.species);
                            match sp.stoichiometry {
                                None => species,
                                Some(x) => match x as u8 {
                                    1 => species,
                                    _ => format!("{} {}", x, species),
                                },
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" + "),
                    match &reac.lower_bound {
                        Some(s) => params[s].value.unwrap(),
                        _ => -1000.,
                    },
                    match &reac.upper_bound {
                        Some(s) => params[s].value.unwrap(),
                        _ => 1000.,
                    },
                )
                .unwrap();
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(243))))
                    .unwrap();
                match &reac.name {
                    Some(name) => writeln!(&mut stdout, " ({})", name).unwrap(),
                    None => writeln!(&mut stdout, " (unnamed)").unwrap(),
                }
            });

            // Each species is formatted to (with colors):
            //     (Species) species.id: species.compartment (species.name)
            model.species.iter().for_each(|(id, met)| {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(243))))
                    .unwrap();
                write!(&mut stdout, "(Species)").unwrap();
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                    .unwrap();
                write!(&mut stdout, " {}", id).unwrap();
                stdout.reset().unwrap();
                write!(&mut stdout, ": {}", met.compartment).unwrap();
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(243))))
                    .unwrap();
                match &met.name {
                    Some(name) => writeln!(&mut stdout, " ({})", name).unwrap(),
                    None => writeln!(&mut stdout, " (unnamed)").unwrap(),
                }
            })
        }
        Err(e) => println!("fdSBML error: {}", e),
    }
}
