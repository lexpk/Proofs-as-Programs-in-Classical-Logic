#[path = "encoding.rs"]
mod encoding;

use tptp::visitor::Visitor;
use tptp::{top, TPTPIterator};

use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Dependencies {
    paths: Vec<String>,
    include_root: Option<String>,
}
impl Dependencies {
    fn new(input_file: String, include_root: Option<String>) -> Dependencies {
        match include_root {
            Some(ref dir) => {
                if !Path::new(&dir).exists() {
                    panic!("Path does not exists: {}", dir);
                }
            }
            _ => {}
        }
        let mut result = Dependencies {
            paths: vec![input_file.clone()],
            include_root: include_root,
        };
        if !Path::new(&input_file).exists() {
            panic!("File does not exists: {}", input_file);
        }
        let path = Path::new(&input_file);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {}
        }
        let bytes = s.as_bytes();
        let mut parser = TPTPIterator::<()>::new(bytes);
        for input in &mut parser {
            let input = input.expect(&format!("syntax error in {}", input_file));
            result.visit_tptp_input(&input);
        }
        return result;
    }
}

impl<'a> Visitor<'a> for Dependencies {
    fn visit_include(&mut self, include: &top::Include<'a>) {
        let path = match self.include_root {
            Some(ref dir) => dir.clone(),
            None => String::new(),
        } + &include.file_name.0.0.to_string();
        self.paths.push(path.clone());
        let mut recursive_result =
            Dependencies::new(path, self.include_root.clone());
        self.paths.append(&mut recursive_result.paths)
    }
    

    fn visit_tptp_input(&mut self, input: &top::TPTPInput<'a>) {
        match input {
            top::TPTPInput::Annotated(_) => {}
            top::TPTPInput::Include(include) => self.visit_include(include),
        }
    }
}

pub struct Problem {
    input: Vec<Vec<u8>>,
    dependencies: Dependencies,
}

impl Problem {
    pub fn new(input_file: String, include_root: Option<String>) -> Problem {
        let mut result = Problem {
            input: vec![],
            dependencies: Dependencies::new(input_file, include_root),
        };
        for arg in result.dependencies.paths.iter() {
            let path = Path::new(&arg);
            let display = path.display();

            let mut file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };

            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", display, why),
                Ok(_) => {}
            }
            result.input.push(s.as_bytes().to_vec());
        }
        return result;
    }

    pub fn translate(&self) -> encoding::Encoding {
        let mut result = encoding::Encoding::new();
        for arg in self.input.iter() {
            result += encoding::Encoding::from_tptp(arg)
        }
        return result;
    }
}
