use std::collections::HashMap;
use std::fs;

static LIBRARY_DIR: &str = "/home/lsund/Media/audio/library";

pub struct Library {
    content: HashMap<u32, String>,
}

pub fn make() -> Library {
    let mut ret = Library {
        content: HashMap::new(),
    };
    let mut i = 0;
    for path in fs::read_dir(LIBRARY_DIR).unwrap() {
        let x = path.unwrap().path().display().to_string();
        ret.content.insert(i, x);
        i += 1;
    }
    return ret;
}

impl Library {
    pub fn list(&self) -> String {
        return self.content.iter().fold("".to_owned(), |acc, (i, x)| {
            acc + &format!("{} {}\n", i, x)
        });
    }
}
