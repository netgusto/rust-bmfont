#![crate_type = "lib"]
#![crate_name = "bmfont"]

extern crate xml;

#[derive(Debug)]
pub struct BmChar {
    id: i32,
    page: i32,
    x: i32,
    y: i32,
    xoffset: i32,
    yoffset: i32,
    width:i32,
    height: i32,
    xadvance: i32,
    chnl: i32
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

#[derive(Debug)]
pub struct BmPage {
    id: i32,
    file: String
}

impl BmPage {
    pub fn new() -> Self {
        BmPage {
            id: 0,
            file: String::new()
        }
    }
}

#[derive(Debug)]
pub struct BmFont {
    pages: Vec<BmPage>,
    chars: Vec<BmChar>
}

pub fn parse(xmlstr: String) -> BmFont {

    use xml::reader::{ EventReader, XmlEvent };

    let mut chars : Vec<BmChar> = Vec::new();
    let mut pages : Vec<BmPage> = Vec::new();

    let parser = EventReader::new(xmlstr.as_bytes());

    for e in parser {

        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {

                match &name.local_name.as_ref() {
                    &"char" => {
                        let mut chr: BmChar = BmChar::new();

                        for attr in attributes {
                            let name = attr.name.local_name;
                            let value = attr.value;

                            match &name.as_ref() {
                                &"id" => chr.id = value.parse::<i32>().unwrap(),
                                &"page" => chr.page = value.parse::<i32>().unwrap(),
                                &"x" => chr.x = value.parse::<i32>().unwrap(),
                                &"y" => chr.y = value.parse::<i32>().unwrap(),
                                &"xoffset" => chr.xoffset = value.parse::<i32>().unwrap(),
                                &"yoffset" => chr.yoffset = value.parse::<i32>().unwrap(),
                                &"width" => chr.width = value.parse::<i32>().unwrap(),
                                &"height" => chr.height = value.parse::<i32>().unwrap(),
                                &"xadvance" => chr.xadvance = value.parse::<i32>().unwrap(),
                                &"chnl" => chr.chnl = value.parse::<i32>().unwrap(),
                                _ => {}
                            }
                        }
                        chars.push(chr);
                    }
                    &"page" => {
                        let mut page: BmPage = BmPage::new();
                        for attr in attributes {
                            let name = attr.name.local_name;
                            let value = attr.value;
                            match &name.as_ref() {
                                &"id" => page.id = value.parse::<i32>().unwrap(),
                                &"file" => page.file = value,
                                _ => {}
                            }
                        }
                        pages.push(page);
                    }
                    _ => {}
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

