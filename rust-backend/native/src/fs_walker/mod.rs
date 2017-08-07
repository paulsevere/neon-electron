use walkdir::*;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

use search_file::*;
use matches::*;
pub fn walk_path(query: &str, path: &str, max_depth: usize, filter: Vec<&str>) -> MatchSet {
    let mut match_set = MatchSet::new(query);
    let filt = |dir: &DirEntry| -> bool {
        let p = dir.file_name();
        let path = p.to_str().unwrap();
        let mut ret = true;
        for x in filter.clone() {
            if (path.contains(x)) {
                ret = false;
            }
        }
        ret
    };
    let mut walker = WalkDir::new(path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(filt);
    for entry in walker {
        let entry = entry.unwrap();
        let path = entry.path().display();
        if should_read(&entry) {

            let mats = search_file(entry.path(), query);
            match_set.search_count += 1;
            if mats.len() > 0 {
                match_set.add_matches(
                    entry.path().to_str().unwrap_or("can't parse filename"),
                    mats,
                );
            }

        }
    }
    match_set
}


fn should_read(entry: &DirEntry) -> bool {

    let file_type = entry.file_type();
    !(file_type.is_dir() || is_exec(entry))


}

fn is_exec(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(data) => data.mode() & 0o111 != 0,
        _ => false,
    }
}