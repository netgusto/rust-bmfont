#![crate_type = "lib"]
#![crate_name = "bmfont"]

extern crate xml;

use std::collections::HashMap;
use std::path::{ PathBuf };

#[derive(Debug, Default)]
pub struct BmChar {
    pub id: u32,
    pub page: u32,
    pub x: i32,
    pub y: i32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub width:i32,
    pub height: i32,
    pub xadvance: i32,
    pub chnl: i32
}

impl BmChar {
    pub fn new() -> Self {
        BmChar {
            id: 0,
            page: 0,
            x: 0,
            y: 0,
            xoffset: 0,
            yoffset: 0,
            width: 0,
            height: 0,
            xadvance: 0,
            chnl: 0
        }
    }
}

#[derive(Debug, Default)]
pub struct BmPage {
    id: i32,
    pub file: PathBuf
}

impl BmPage {
    pub fn new() -> Self {
        BmPage {
            id: 0,
            file: PathBuf::new()
        }
    }
}

#[derive(Debug)]
pub struct BmFont {
    pub pages: Vec<BmPage>,
    pub chars: HashMap<u32, BmChar>
}

impl BmFont {
    pub fn get_pages(&self) -> &Vec<BmPage> {
        &self.pages
    }
}

pub fn parse(fontdescriptorpath: PathBuf) -> BmFont {

    use std::io::prelude::*;
    use std::fs::File;
    use xml::reader::{ EventReader, XmlEvent };

    // let fontdescriptordir: &Path;

    // match fontdescriptorpath.parent() {
    //     Some(p) => fontdescriptordir = p,
    //     _ => panic!("BmFont: Cannot determine font descriptor directory: {:?}", fontdescriptorpath)
    // }

    // println!("ROOT DIR: {:?}", fontdescriptordir);

    let mut f = File::open(&fontdescriptorpath).unwrap();

    let mut xmlstr = String::new();
    f.read_to_string(&mut xmlstr).unwrap();

    let mut chars : HashMap<u32, BmChar> = HashMap::new();
    let mut pages : Vec<BmPage> = Vec::new();

    let parser = EventReader::new(xmlstr.as_bytes());

    for e in parser {

        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {

                match name.local_name.as_ref() {
                    "char" => {
                        let mut chr: BmChar = BmChar::new();

                        for attr in attributes {
                            let name = attr.name.local_name;
                            let value = attr.value;

                            match name.as_ref() {
                                "id" => chr.id = value.parse::<u32>().unwrap(),
                                "page" => chr.page = value.parse::<u32>().unwrap(),
                                "x" => chr.x = value.parse::<i32>().unwrap(),
                                "y" => chr.y = value.parse::<i32>().unwrap(),
                                "xoffset" => chr.xoffset = value.parse::<i32>().unwrap(),
                                "yoffset" => chr.yoffset = value.parse::<i32>().unwrap(),
                                "width" => chr.width = value.parse::<i32>().unwrap(),
                                "height" => chr.height = value.parse::<i32>().unwrap(),
                                "xadvance" => chr.xadvance = value.parse::<i32>().unwrap(),
                                "chnl" => chr.chnl = value.parse::<i32>().unwrap(),
                                _ => {}
                            }
                        }
                        //chars.push(chr);
                        chars.insert(chr.id, chr);
                    }
                    "page" => {
                        let mut page: BmPage = BmPage::new();
                        for attr in attributes {
                            let name = attr.name.local_name;
                            let value = attr.value;
                            match name.as_ref() {
                                "id" => page.id = value.parse::<i32>().unwrap(),
                                "file" => {
                                    page.file = fontdescriptorpath.with_file_name(value);
                                },
                                _ => {}
                            }
                        }
                        pages.push(page);
                    }
                    "info" | "common" | _ => { }
                }
            }

            Err(e) => {
                println!("Error: {}", e);
                break;
            }

            _ => {}
        }
    }

    BmFont { pages: pages, chars: chars }
}

