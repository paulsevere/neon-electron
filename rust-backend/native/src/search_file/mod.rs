

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
pub struct Match {
    pub line: usize,
    pub position: usize,
    pub text: String,
}
impl Match {
    pub fn new(line: usize, position: usize, text: String) -> Self {
        Self {
            line: line,
            position: position,
            text: text,
        }
    }
}

pub fn search_file(path: &Path, query: &str) -> Vec<Match> {
    let mut matches: Vec<Match> = vec![];
    let f = File::open(path).unwrap();
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap_or(String::from(""));

        if let Some(mat) = l.find(query) {
            let cmatch = Match::new(num, mat, String::from(l));
            matches.push(cmatch);
        }
    }
    matches
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn it_works() {
        let path = Path::from("Cargo.toml".to_string());
        let matches = ::search_file(path, "dep");
        dump!(matches);
    }
}
