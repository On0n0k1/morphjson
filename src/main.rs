use std::collections::HashMap;
use std::fs::File;

use serde::{ Deserialize, Serialize };
use rand::{thread_rng, Rng, rngs::ThreadRng};

#[derive(Debug, Serialize, Deserialize)]
struct Entry{
    pub name: String,
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_ascii_lowercase() == other.name.to_ascii_lowercase()
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase())
    }
}


impl From<String> for Entry {
    fn from(name: String) -> Self {
        Entry { name }
    }
}

impl From<&str> for Entry {
    fn from(name: &str) -> Self {
        let name: String = name.into();
        Entry { name }
    }
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Input{
    Values(Vec<Entry>)
}

impl Input {
    fn get(&self) -> &Vec<Entry> {
        match &self {
            Input::Values(entries) => entries
        }
    }

    /// Even if the entries are sorted, the Hashmap will not be.
    /// 
    /// So this method is currently useless.
    #[allow(unused)]
    fn sorted(self) -> Self {
        let Input::Values(mut entries) = self;


        entries.sort();

        Input::Values(entries)
    }
}

// impl From<D: Iterator> for Input where D::Item: Into<Entry> {
//     fn from(entries: D) -> Self {
        
//     }
// }

impl<D: Into<Vec<Entry>>> From<D> for Input {
    fn from(entries: D) -> Self {
        let entries: Vec<Entry> = entries.into();
        Input::Values(entries)
    }
}



fn log(msg: &str) {
    println!("{msg}");
}

fn load(rng: &mut ThreadRng, file: &str, output: &mut HashMap<String, u32>) {
    let text = std::fs::read_to_string(file).unwrap();

    log("Converting JSON...");
    let input: Input = serde_json::from_str(&text).unwrap();

    // let input: Input = input.sorted();


    log("Including Entries...");
    for i in input.get() {
        let times: u32 = rng.gen_range(1..1100);
        let times: u32 = if times > 999 { 999 } else { times };
        
        output.insert(i.name.clone(), times);
    }

    log("Loading {file} successful");
}

fn main() {
    let mut rng: ThreadRng = thread_rng();
    
    let mut output: HashMap<String, u32> = HashMap::new();

    load(&mut rng, "data.json", &mut output);
    load(&mut rng, "data2.json", &mut output);
    load(&mut rng, "data3.json", &mut output);

    log("Loading/Creating File");
    let file = File::create("names.json").expect("Failed to Load/Create File");

    log("Saving Entries...");
    serde_json::to_writer_pretty(&file, &output).expect("Failed to deserialize entries...");
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sorting(){
        let mut entries: Vec<Entry> = [
            "Alan",
            "Alarick",
            "alan",
            "roderick",
            "Sam",
            "Albedo",
        ].map(Entry::from).into();

        entries.sort();

        assert_eq!(
            entries,
            Vec::from([
                "Alan",
                "alan",
                "Alarick",
                "Albedo",
                "roderick",
                "Sam",
            ].map(Entry::from))
        );

    }
}