#![allow(warnings)]
#[macro_use]
extern crate neon;
#[macro_use]
extern crate dump;

extern crate walkdir;

use neon::vm::{Call, JsResult};
use neon::js;
use neon::js::{Object, JsArray, JsString, JsInteger, JsObject, JsNumber, JsFunction};
use std::fs::File;
use std::io::Read;
use neon::js::Value;
mod search_file;
use search_file::*;
mod fs_walker;
use fs_walker::*;
mod matches;

fn hello(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let args = call.arguments;


    let arg = args.get(scope, 0).unwrap();
    let cb = args.get(scope, 1).unwrap();

    let arg_string = arg.to_string(scope).unwrap().value();
    let matches = walk_path(arg_string.as_str(), "./", 4, vec!["target", "node_modules"]);


    dump!(matches.len());
    let ret = JsObject::new(scope);

    let obj = JsObject::new(scope);

    for (key, value) in matches.matches.iter() {
        let marr = JsArray::new(scope, value.len() as u32);
        for (n, mat) in value.iter().enumerate() {
            let matt = JsObject::new(scope);
            matt.set("line", JsInteger::new(scope, mat.line as i32));
            matt.set("position", JsInteger::new(scope, mat.position as i32));
            matt.set("text", JsString::new(scope, mat.text.as_str()).unwrap());

            marr.set(n as u32, matt);
        }
        ret.set(key.as_str(), marr);
    }
    obj.set("count", JsInteger::new(scope, matches.len() as i32));
    obj.set("matches", ret);
    println!("{}", matches.search_count);
    Ok(obj) // ret.set(scope, "count", JsInteger::new(scope, matches.len() as i32));

}

register_module!(m, {
    m.export("hello", hello)
});
