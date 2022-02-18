use std::fs::File;
use std::io::{BufReader};
use std::ops::Add;
use crate::colors::rgb;
use crate::flag::{Flag};
use clap::{arg, Command};

mod colors;
mod flag;

fn main() {
    let args = Command::new("ironic_flags")
        .version( "0.1.0")
        .author("Niklas Schmidt <36010519+neon-js@users.noreply.github.com>")
        .about("Shows colored flags in ansi terminals.")
        .arg(
            arg!(-f --file <FILE> "File that contains the flag definitions")
        )
        .arg(
            arg!(-n --flag <FLAGNAME> "Name of flag to show")
        )
        .arg(
            arg!(-w --width [WIDTH] "Width of flag. If omitted, it's set to ~ height * 2.")
        )
        .arg(
            arg!(-h --height [HEIGHT] "Height of flag. If omitted, it's set the minimal value that's necessary to show the flag")
        )
        .get_matches();

    let file_name = args.value_of("file")
        .expect("Argument file is required!");

    let flag_name = args.value_of("flag")
        .expect("Argument flag-name is required!");

    let height = args.value_of("height")
        .or(Some("0"))
        .and_then(|value|value.parse::<usize>().ok());

    let width = args.value_of("width")
        .and_then(|value|value.parse::<usize>().ok());

    if height.is_none() {
        eprintln!("Error: Invalid height given.");
        return;
    }

    let file_reader = File::open(&file_name)
        .map(|file| BufReader::new(file));

    if file_reader.is_err() {
        eprintln!("Error: Cannot read file. Check if it exists and is readable.");
        return;
    }

    let flags: serde_json::Result<Vec<Flag>> = serde_json::from_reader(file_reader.unwrap());
    if flags.is_err() {
        eprintln!("Error: Cannot parse file. Check if it is valid.");
        return;
    }

    let flags = flags.unwrap();

    let filtered_flags = flags
        .iter()
        .filter(|flag| flag.name == flag_name)
        .collect::<Vec<&Flag>>();
    let flag = filtered_flags.first();

    if flag.is_none() {
        eprintln!("Error: Cannot find flag in given file. Make sure that it exists in file.");
        return;
    }

    println!("{}", generate_flag_text(flag.unwrap(), height.unwrap(), width));
}

/// Generates text that shows given flag in an ansi terminal.
///
/// # Arguments
///
/// * `flag` - Flag that should be displayed
/// * `height` - _minimal_ height of the flag to display
/// * `width` - If given, width of the flag to display. If omitted, its set to ~ `2 * height`.
fn generate_flag_text(flag: &Flag, height: usize, width: Option<usize>) -> String
{
    let relation_sum = flag.parts.iter()
        .map(|part| part.relation)
        .sum::<usize>();

    let mut height_factor = height as f32 / relation_sum as f32;
    if height_factor < 1.0 {
        // Prevent "invisible" flags if target height < necessary height
        height_factor = 1.0;
    }

    let width = width.unwrap_or(relation_sum * (height_factor as usize) * 4);

    flag.parts.iter()
        .map(|part| {
            format!("{}{}{}\n", part.color_string(), " ".repeat(width), colors::reset())
                .repeat((part.relation as f32 * height_factor) as usize)
        })
        .collect::<String>()
        .add(colors::reset())
}