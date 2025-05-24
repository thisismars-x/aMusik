#![feature(string_remove_matches)]

use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{self, Clear, ClearType, size},
};
use dialoguer::Select;
use home::home_dir;
use indicatif::{ProgressBar, ProgressStyle};
use mp3_duration;
use rand::{rng, seq::SliceRandom};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::{self, File};
use std::io::BufReader;
use std::io::stdout;

type Songs = Vec<String>;
fn scan() -> Songs {
    let mut list = Songs::new();
    if let Some(mut path) = home_dir() {
        path.push("Music");

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    if file.is_file() {
                        let file = file.display().to_string();
                        if file.ends_with("mp3") {
                            list.push(file)
                        }
                    }
                }
            }
        }
    }

    list
}

fn main() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0),).unwrap();
    let (cols, rows) = size().unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut songs = scan();
    let modes = ["Shuffle", "Single Loop", "Playlist Loop", "Make Playlist"];
    let modes_ = [
        "Shuffle".yellow(),
        "Single Loop".yellow(),
        "Playlist Loop".yellow(),
        "Make Playlist".yellow(),
    ];

    println!("{}", "[aMusik]".italic().green());
    let mode = Select::new()
        .with_prompt("Select Mode")
        .items(&modes_)
        .interact()
        .unwrap();

    match modes[mode] {
        "Shuffle" => {
            let mut rngg = rng();

            loop {
                songs.shuffle(&mut rngg);
                songs.iter().for_each(|item| {
                    sink.append(Decoder::new(BufReader::new(File::open(item).unwrap())).unwrap());
                    let file = File::open(item).unwrap();
                    let duration = mp3_duration::from_file(&file).unwrap();

                    let pb = ProgressBar::new(duration.as_secs() as u64);
                    pb.set_style(
                        ProgressStyle::with_template(
                            "[{elapsed_precise}] {bar:40.cyan/blue} [{eta_precise}]",
                        )
                        .unwrap()
                        .progress_chars("##-"),
                    );

                    let basepath = format!("{}/Music/", home_dir().unwrap().display());
                    let mut itemname = item.clone();
                    itemname.remove_matches(basepath.as_str());
                    itemname.remove_matches(".mp3");

                    let display = format!("Now playing '{itemname}'");
                    println!("\n{}", display.black().on_green().bold());
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        if pb.position() >= duration.as_secs() {
                            break;
                        }
                        pb.set_position(pb.position() + 2);
                    }
                })
            }
        }
        "Single Loop" => {
            let mut songs_colored = Vec::new();
            songs.iter().for_each(|item| {
                let mut x = item.clone();
                let basepath = format!("{}/Music/", home_dir().unwrap().display());
                x.remove_matches(&basepath);
                x.remove_matches(".mp3");
                songs_colored.push(x.yellow().to_string())
            });
            let song = Select::new()
                .with_prompt("Select song")
                .items(&songs_colored)
                .default(0)
                .interact()
                .unwrap();
            let file = File::open(&songs[song]).unwrap();
            let duration = mp3_duration::from_file(&file).unwrap();

            let basepath = format!("{}/Music/", home_dir().unwrap().display());
            let mut itemname = songs[song].clone();
            itemname.remove_matches(basepath.as_str());
            itemname.remove_matches(".mp3");

            let display = format!("Now playing '{itemname}'");
            println!("\n{}", display.black().on_green().bold());
            loop {
                sink.append(
                    Decoder::new(BufReader::new(File::open(&songs[song]).unwrap())).unwrap(),
                );
                let pb = ProgressBar::new(duration.as_secs() as u64);
                pb.set_style(
                    ProgressStyle::with_template(
                        "[{elapsed_precise}] {bar:40.cyan/blue} [{eta_precise}]",
                    )
                    .unwrap()
                    .progress_chars("##-"),
                );

                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    if pb.position() >= duration.as_secs() {
                        break;
                    }
                    pb.set_position(pb.position() + 2);
                }
            }
        }
        "Playlist Loop" => loop {
            songs.iter().for_each(|item| {
                sink.append(Decoder::new(BufReader::new(File::open(item).unwrap())).unwrap());
                let file = File::open(item).unwrap();
                let duration = mp3_duration::from_file(&file).unwrap();

                let pb = ProgressBar::new(duration.as_secs() as u64);
                pb.set_style(
                    ProgressStyle::with_template(
                        "[{elapsed_precise}] {bar:40.cyan/blue} [{eta_precise}]",
                    )
                    .unwrap()
                    .progress_chars("##-"),
                );

                let basepath = format!("{}/Music/", home_dir().unwrap().display());
                let mut itemname = item.clone();
                itemname.remove_matches(basepath.as_str());
                itemname.remove_matches(".mp3");

                let display = format!("Now playing '{itemname}'");
                println!("\n{}", display.black().on_green().bold());
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    if pb.position() >= duration.as_secs() {
                        break;
                    }
                    pb.set_position(pb.position() + 2);
                }
            })
        },

        "Make Playlist" => {
            let mut songs_colored = Vec::new();
            songs.iter().for_each(|item| {
                let mut x = item.clone();
                let basepath = format!("{}/Music/", home_dir().unwrap().display());
                x.remove_matches(&basepath);
                x.remove_matches(".mp3");
                songs_colored.push(x.yellow().to_string())
            });
            let songs_ind = dialoguer::MultiSelect::new()
                .with_prompt("Select songs")
                .items(&songs_colored)
                .interact()
                .unwrap();

            for ind in songs_ind {
                sink.append(
                    Decoder::new(BufReader::new(File::open(&songs[ind]).unwrap())).unwrap(),
                );
                let file = File::open(&songs[ind]).unwrap();
                let duration = mp3_duration::from_file(&file).unwrap();

                let pb = ProgressBar::new(duration.as_secs() as u64);
                pb.set_style(
                    ProgressStyle::with_template(
                        "[{elapsed_precise}] {bar:40.cyan/blue} [{eta_precise}]",
                    )
                    .unwrap()
                    .progress_chars("##-"),
                );

                let basepath = format!("{}/Music/", home_dir().unwrap().display());
                let mut itemname = songs[ind].clone();
                itemname.remove_matches(basepath.as_str());
                itemname.remove_matches(".mp3");

                let display = format!("Now playing '{itemname}'");
                println!("\n{}", display.black().on_green().bold());
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    if pb.position() >= duration.as_secs() {
                        break;
                    }
                    pb.set_position(pb.position() + 2);
                }
            }
        }
        _ => {}
    }
}
