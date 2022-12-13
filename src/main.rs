extern crate quick_xml;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use quick_xml::events::Event;
use quick_xml::Reader;
// Import QName
use quick_xml::name::QName;

// Function that calls url and returns xml
fn get_xml(url: &str) -> String {
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    body
}

// Function that parses xml out <li> element and returns a vector of strings
fn parse_xml(xml: &str) -> Vec<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut links = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                if e.name() == QName(b"li") {
                    let mut link = String::new();
                    for attr in e.attributes() {
                        let attr = attr.unwrap();
                        if attr.key == QName(b"href") {
                            link = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                    links.push(link);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    links
}

// Create reader object from xml string
fn create_reader(xml: &str) -> Reader<&[u8]> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    reader
}

// Function that parses xml up to <li> and returns reader
fn parses_to_li<'a>(reader: &'a mut Reader<&'a [u8]>) -> &'a Reader<&[u8]> {
    match reader.read_event() {
        Ok(Event::Start(ref e)) => {
            if e.name() == QName(b"li") {
                return reader;
            } else {
                return parses_to_li(reader);
            }
        }
        Ok(Event::Eof) => panic!("EOF"),
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        _ => panic!("Error"),
    }
}

fn main() {
    println!("Hello, world!");
}
