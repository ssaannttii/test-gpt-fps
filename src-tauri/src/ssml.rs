use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Cursor;

pub fn ssml_to_plain(text: &str) -> Result<String> {
    let mut reader = Reader::from_reader(Cursor::new(text.as_bytes()));
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut output = String::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(_) | Event::End(_) | Event::Eof => {
                if matches!(reader.buffer_position(), 0) {
                    break;
                }
            }
            Event::Text(e) => {
                output.push_str(&e.unescape()?.trim());
                output.push(' ');
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(output.trim().to_string())
}
