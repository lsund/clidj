use std::collections::BTreeMap;
use std::fs;

static MAX_SONGS: usize = 15;

pub struct Library {
    pub content: BTreeMap<u32, String>,
    load_index: Option<u32>,
}

pub fn make(library_dir: &str) -> Library {
    let mut ret = Library {
        content: BTreeMap::new(),
        load_index: None,
    };
    let mut i = 0;
    for path in fs::read_dir(library_dir).unwrap() {
        let x = path.unwrap().path().display().to_string();
        ret.content.insert(i, x);
        i += 1;
    }
    return ret;
}

impl Library {
    pub fn list(&self) -> String {
        return self
            .content
            .iter()
            .take(MAX_SONGS)
            .fold("".to_owned(), |acc, (i, x)| acc + &format!("{} {}\n", i, x));
    }

    pub fn load(&mut self, x: u32) {
        self.load_index = Some(x);
    }
}
