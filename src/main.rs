use std::env;
use std::fs::File;
use std::io::{BufReader};
use std::ops::Add;
use crate::colors::rgb;
use crate::flag::{Flag};

mod colors;
mod flag;

fn main() {
    let information = information_from_args();
    if information.is_none() {
        eprintln!("Error: Wrong usage.");
        eprintln!("       Call ./ironic_flags <FLAGS_FILE> <FLAG_NAME>");
        return;
    }

    let (file_name, flag_name) = information.unwrap();

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

    /* TODO: Make height and width variable */
    println!("{}", generate_flag_text(flag.unwrap(), 5));
}

/// Generates text that shows given flag in an ansi terminal.
///
/// # Arguments
///
/// * `flag` - Flag that should be displayed
/// * `height` - _minimal_ height of the flag to display
fn generate_flag_text(flag: &Flag, height: usize) -> String
{
    let relation_sum = flag.parts.iter()
        .map(|part| part.relation)
        .sum::<usize>();

    let mut height_factor = height as f32 / relation_sum as f32;
    if height_factor < 1.0 {
        // Prevent "invisible" flags if target height < necessary height
        height_factor = 1.0;
    }

    let width = relation_sum * 4;

    flag.parts.iter()
        .map(|part| {
            format!("{}{}{}\n", part.color_string(), " ".repeat(width), colors::reset())
                .repeat((part.relation as f32 * height_factor) as usize)
        })
        .collect::<String>()
        .add(colors::reset())
}

fn information_from_args() -> Option<(String, String)>
{
    let mut args = env::args();
    args.next()?; /* Program name */

    Some((args.next()?, args.next()?))
}