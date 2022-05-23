use crate::parser::Website;
use std::collections::HashMap;
use tera::{ Context, Tera};
use std::fs::{self, File};
use std::io::Write;
use crate::config::TEMPLATE_FILE_PATH;

pub fn get_path_of_file(path: &str) -> String {
    let mut s = path.split("/").collect::<Vec<&str>>();
    s.pop();
    s.join("/")
}

pub fn create_directories(path_str: &str) {
    fs::create_dir_all(path_str)
        .expect(format!("Could not create directory: {}", path_str).as_str());
}

pub fn create_file(path_str: &str, content: &[u8]) {
    let file_path = get_path_of_file(path_str);
    create_directories(&file_path);
    let mut f =
        File::create(path_str)
            .expect(format!("Could not create file: {}", path_str).as_str());
    f.write_all(content)
        .expect(format!("Could not write to file: {}", path_str).as_str());
}

pub struct Renderer {
    instance: Tera,
    list: HashMap<String, Website>,
}
impl Renderer {
    pub fn init() -> Self {
        let mut ret = Renderer {
            instance: Tera::new(TEMPLATE_FILE_PATH).expect("Could not create Tera instance"),
            list: HashMap::new(),
        };
        ret.instance.autoescape_on(vec![]);
        ret
    }
    pub fn cache(&mut self, path_str: String, data: Website) {
        self.list.insert(path_str, data);
    }
    pub fn render(&self, path: &str, data: &Website) {
        let mut ctx = Context::from_serialize(data).expect("Could not create context");
        let r = self.instance.render("index.html", &mut ctx)
            .expect(format!("Could not render template: {}", path).as_str());
        
        create_file(path, r.as_bytes());
    }
	pub fn render_all(&self) {
		for (path, data) in &self.list {
			self.render(path, data);
		}
	}
}
