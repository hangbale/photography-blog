use crate::config::Website;
use std::collections::HashMap;
use tera::{ Context, Tera};
use std::fs::{self, File};
use std::io::Write;
use crate::config::{
    TEMPLATE_FILE_PATH,
    PUBLIC_PATH
};

pub fn create_directories(path_str: &str) {
    fs::create_dir_all(path_str)
        .expect(format!("Could not create directory: {}", path_str).as_str());
}

pub fn create_file(path_str: &str, content: &[u8]) {
    let final_path = PUBLIC_PATH.to_string() + path_str;
    create_directories(&final_path);
    let file_path = final_path + "/index.html";
    let mut f =
        File::create(file_path)
            .expect(format!("Could not create file: {}", path_str).as_str());
    f.write_all(content)
        .expect(format!("Could not write to file: {}", path_str).as_str());
}

pub struct Renderer <'a> {
    instance: Tera,
    list: HashMap<&'a str, &'a Website>
}
impl<'a> Renderer<'a> {
    pub fn init() -> Self {
        let mut ret = Renderer {
            instance: Tera::new(TEMPLATE_FILE_PATH).expect("Could not create Tera instance"),
            list: HashMap::new(),
        };
        ret.instance.autoescape_on(vec![]);
        ret
    }

    pub fn render(&self, path: &str, data: &Website) {
        let mut ctx = Context::from_serialize(data).expect("Could not create context");
        // ctx.insert("extra", &data.1);
        let r = self.instance.render("index.html", &mut ctx)
            .expect(format!("Could not render template: {}", path).as_str());
        
        create_file(path, r.as_bytes());
    }
	pub fn render_all(&self) {
		for (path, data) in &self.list {
			self.render(*path, *data);
		}
	}
    pub fn transfrom_config_to_list(&mut self, config: &'a Website) {
        if let Some(path_str) = &config.path {
            self.list.insert(&path_str, config);
            if let Some(children) = &config.children {
                for child in children {
                    self.transfrom_config_to_list(&child);
                }
            }
        }
    
    }
}
