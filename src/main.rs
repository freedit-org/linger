use bincode::{config::standard, Decode, Encode};
use serde::Deserialize;
use std::{fmt::Display, io, time::Duration};
use ureq::Agent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_local_dir().expect("Couldn't get local config directory");
    let config_path = config_dir.join("linger");
    let config = sled::Config::default().path(config_path);
    let db = config.open()?;

    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(8))
        .timeout_write(Duration::from_secs(8))
        .build();

    // let word = std::env::args().nth(1).expect("Usage: ric <word>");

    loop {
        println!("Enter word: ('e'/'exit' to quit)");
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        if word.trim() == "exit" || word.trim() == "e" {
            println!("Goodbye!");
            break;
        }

        let words = if let Some(v) = db.get(&word)? {
            let (decoded, _): (Vec<Word>, usize) = bincode::decode_from_slice(&v, standard())?;
            decoded
        } else {
            let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
            let web = agent.get(&url).call()?.into_json()?;
            let encoded = bincode::encode_to_vec(&web, standard())?;
            db.insert(word, encoded)?;
            web
        };

        for i in words {
            println!("{i}")
        }
    }

    Ok(())
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== {} ==\n", self.word)?;

        for p in &self.phonetics {
            if let Some(text) = &p.text {
                writeln!(f, "{}", text)?;
            }

            writeln!(f, "{}", p.audio)?;
        }

        writeln!(f)?;

        for m in &self.meanings {
            writeln!(f, "【{}】", m.part_of_speech)?;
            for (i, d) in m.definitions.iter().enumerate() {
                writeln!(f, "{i}. {}", d.definition)?;
                if !d.antonyms.is_empty() {
                    writeln!(f, "antonyms: {:?}", d.antonyms)?;
                }
                if !d.synonyms.is_empty() {
                    writeln!(f, "synonyms: {:?}", d.synonyms)?;
                }
            }

            if !m.antonyms.is_empty() {
                writeln!(f, "antonyms: {:?}", m.antonyms)?;
            }

            if !m.synonyms.is_empty() {
                writeln!(f, "synonyms: {:?}", m.synonyms)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Decode, Encode)]
struct Word {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
    license: License,
    #[serde(rename(deserialize = "sourceUrls"))]
    source_urls: Vec<String>,
}
#[derive(Debug, Deserialize, Decode, Encode)]

struct License {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Decode, Encode)]
struct Phonetic {
    text: Option<String>,
    audio: String,
    #[serde(rename(deserialize = "sourceUrl"))]
    source_url: Option<String>,
    license: Option<License>,
}

#[derive(Debug, Deserialize, Decode, Encode)]
struct Meaning {
    #[serde(rename(deserialize = "partOfSpeech"))]
    part_of_speech: String,
    definitions: Vec<Definition>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Deserialize, Decode, Encode)]
struct Definition {
    definition: String,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}
