use std::{io, path::PathBuf};

use fields::Fields;
use hash::hash;
use generate::from_hash::HashGen;

mod fields;
pub mod hash;
mod generate;
mod svg;

pub fn save_random_icon_from_hashed_string(
    string: String,
    target_dir: PathBuf,
    filename: Option<String>
) -> Result<(), io::Error> {
    let hash = hash(string);
    let hash_gen = HashGen::new(hash);
    let fields = Fields::new(hash_gen);

    let filename = filename.unwrap_or(hash.to_string());
    let target_path = target_dir.join(filename);

    fields.save_as_svg_file(target_path)
}

pub fn save_random_icon(
    target_dir: PathBuf,
    filename: Option<String>
) -> Result<(), io::Error> {
    let fields: Fields = Fields::new(rand::rng());

    let filename = filename.unwrap_or("random_icon".to_string());
    let target_path = target_dir.join(filename);

    fields.save_as_svg_file(target_path)
}