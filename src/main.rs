extern crate serde;

use evtx::EvtxParser;
use quick_xml::de::DeError;
use std::{fs, path::PathBuf, process};
use yamato_event_analyzer::detections::configs;
use yamato_event_analyzer::detections::detection;
use yamato_event_analyzer::detections::print::MESSAGES;
use yamato_event_analyzer::omikuji::Omikuji;
use yamato_event_analyzer::toml;

fn main() -> Result<(), DeError> {
    configs::singleton();
    let mut toml = toml::ParseToml::new();
    &toml.read_dir("rules".to_string());

    for rule in toml.rules {
        match rule {
            Ok(_rule) => {
                let mut message = MESSAGES.lock().unwrap();
                if let Some(messages) = _rule.rule.messages {
                    for (key, texts) in messages {
                        message.insert_message(key, texts);
                    }
                }
            }
            Err(_) => (),
        }
    }

    let filepath: String = configs::singleton()
        .args
        .value_of("filepath")
        .unwrap_or("")
        .to_string();
    if filepath != "" {
        parse_file(&filepath);
    }

    Ok(())
}

fn parse_file(filepath: &str) {
    let fp = PathBuf::from(filepath);
    let parser = match EvtxParser::from_path(fp) {
        Ok(pointer) => pointer,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut detection = detection::Detection::new();
    &detection.start(parser);
}

fn output_with_omikuji(omikuji: Omikuji) {
    let fp = &format!("art/omikuji/{}", omikuji);
    let content = fs::read_to_string(fp).unwrap();
    println!("{}", content);
}
