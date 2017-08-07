use std::collections::HashMap;
use search_file::*;
use neon::js::{Object, JsArray, JsString, JsInteger, JsObject, JsNumber};
use neon::scope::*;
use neon::mem::Handle;
use neon::vm::VmResult;


#[derive(Debug)]
pub struct MatchSet {
    query: String,
    pub matches: HashMap<String, Vec<Match>>,
    pub search_count: usize,
}



impl MatchSet {
    pub fn new(query: &str) -> Self {
        Self {
            query: String::from(query),
            matches: HashMap::new(),
            search_count: 0,
        }
    }
    pub fn add_matches(&mut self, path: &str, matches: Vec<Match>) {
        self.matches.insert(String::from(path), matches);
    }
    pub fn len(&self) -> usize {
        self.matches.values().fold(0, |acc, val| acc + val.len())
    }



    // pub fn into_js_obj<'a>(&self, scope: &mut RootScope) -> VmResult<Handle<JsObject>> {
    //     let mut obj = JsObject::new(scope);
    //     obj.set("query", JsString::new(scope, self.query.as_str()).unwrap());
    //     Ok(obj)
    // }
}