use crate::utils::*;
use crate::{fmtln, reader};

pub fn pre_compiler(contents: String, debug: bool) -> Result<String, ()> {
    let mut e: bool = false;
    let a = At::PreCompiler;
    let mut clean_contents: String = contents.split('\n').filter(|l| !l.trim().starts_with(".use")).collect::<Vec<&str>>().join("\n");
    
    for (i, ln) in contents.split('\n').filter_map(|l| l.trim().strip_prefix(".use")).rev().enumerate() {
        let ln = ln.trim();

        if ln.is_empty() {
            logger(Level::Err, &a, fmtln!(i, "`.use` Directive Missing a Path Argument"));
            e = true;
            continue;
        }

        if debug {
            logger(Level::Debug, &a, &format!("Path {i}: {ln:?}"));
        }
        
        let incl_contents = match reader(ln.to_string()) {
            Ok(c) => c + "\n",
            Err(why) => {
                logger(Level::Err, &a, &why);
                e = true;
                continue;
            },
        };

        clean_contents.insert_str(0, &incl_contents);
    }

    println!("{clean_contents}");

    if e { return Err(()); }

    Ok(clean_contents)
}