use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Background Image Location")]
    image: String,

    #[serde(rename = "Guid")]
    guid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicProfileConfig {
    #[serde(rename = "Profiles")]
    profiles: Vec<Profile>,
}

fn get_json(path: &PathBuf) -> Result<DynamicProfileConfig> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let out = serde_json::from_reader(reader).context("Json parsing failed bitch")?;
    Ok(out)
}

fn main() -> Result<()> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        // .multi(true)
        .preview(Some("")) // preview should be specified to enable preview window
        .build()
        .unwrap();
    let item_reader = SkimItemReader::default();

    let dir = PathBuf::from(
        std::env::var("HOME").context("$HOME env variable not set")? + "/Projects/pixel-art",
    );
    let item_strings = read_dir(&dir)
        .context("Couldn't find directory")?
        .map(|f| f.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let to_remove = dir.to_str().unwrap().replace("pixel-art", "");
    let items = item_reader.of_bufread(std::io::Cursor::new(item_strings.replace(&to_remove, "")));
    let selected_item = Skim::run_with(&options, Some(items.clone()))
        .map(|out| out.selected_items)
        .context("didn't select an item")?;

    let item = selected_item.iter().nth(0).unwrap();
    let path = PathBuf::from(
        std::env::var("HOME").unwrap()
            + "/Library/Application Support/iTerm2/DynamicProfiles/dynamic.json",
    );
    let mut json = get_json(&path)?;
    let image = item_strings.lines().find(|v| v.contains(&item.output().to_string())).unwrap();
    json.profiles.get_mut(0).unwrap().image = image.to_string();
    std::fs::write(path, serde_json::to_string_pretty(&json).unwrap())?;

    println!("Changing to: {}", item.output());
    Ok(())
}
