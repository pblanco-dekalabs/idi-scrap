use std::{fs::File, io::BufReader};

use genpdf::{
    elements::{self, Paragraph},
    style::{self, Style},
    Alignment, Document, Element,
};
use serde::Deserialize;

use crate::{errors::RuntimeError, remote::EvidenceData, res::Res};

const TITLE_FONT_SIZE: u8 = 30;
const SUBTITLE_FONT_SIZE: u8 = 24;
const SUBTITLE_FOOT_FONT_SIZE: u8 = 19;

fn blue_hue() -> style::Style {
    style::Style::new().with_color(style::Color::Rgb(84, 141, 212))
}
fn gray_20() -> style::Style {
    style::Style::new().with_color(style::Color::Rgb(128, 128, 128))
}

trait Adder<E: Element + 'static> {
    fn add_to(self, doc: &mut Document);
}
impl<E: Element + 'static> Adder<E> for E {
    fn add_to(self, doc: &mut Document) {
        doc.push(self)
    }
}

trait Sizing {
    fn sized(size: u8) -> Self;
}
impl Sizing for Style {
    fn sized(size: u8) -> Self {
        Style::new().with_font_size(size)
    }
}

#[derive(Debug, Deserialize)]
struct FontMeta {
    font: String,
}

fn get_font_name() -> Res<String> {
    if let Ok(file) = File::open("fonts/manifest.json") {
        let reader = BufReader::new(file);
        let json: Result<FontMeta, serde_json::Error> = serde_json::from_reader(reader);
        match json {
            Ok(meta) => Ok(meta.font),
            Err(err) => {
                eprint!("Errors during manifest parse: {}", err);
                RuntimeError::err(
                    "Bad fonts/manifest.json file! Recover from the repository an example.",
                )
            }
        }
    } else {
        RuntimeError::err("Cannot open fonts/manifest.json!")
    }
}

fn create_document() -> Res<Document> {
    get_font_name().map(|font_name| {
        let font_family = genpdf::fonts::from_files("./fonts", &font_name, None)
            .expect("Failed to load font family");
        genpdf::Document::new(font_family)
    })
}

pub fn gen_pdf(data: EvidenceData, output: String) -> Res<()> {
    if let Ok(mut doc) = create_document() {
        doc.set_title(&output);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        decorator.set_header(|_page| {
            elements::FramedElement::new(elements::Break::default()).styled(gray_20())
        });
        doc.set_page_decorator(decorator);
        elements::Break::new(10).add_to(&mut doc);
        Paragraph::default()
            .aligned(Alignment::Center)
            .styled_string("D", blue_hue())
            .styled_string("atos de evidencia", gray_20())
            .styled(Style::sized(TITLE_FONT_SIZE))
            .add_to(&mut doc);
        Paragraph::default()
            .aligned(Alignment::Center)
            .styled_string(data.month[0..1].to_string(), blue_hue())
            .styled_string(data.month[1..data.month.len()].to_string(), gray_20())
            .styled_string(" 20", gray_20())
            .styled_string(data.year.to_string(), gray_20())
            .styled(Style::sized(TITLE_FONT_SIZE))
            .add_to(&mut doc);
        elements::Break::new(10).add_to(&mut doc);
        Paragraph::default()
            .aligned(Alignment::Center)
            .string(data.author)
            .styled(Style::sized(SUBTITLE_FONT_SIZE))
            .add_to(&mut doc);
        Paragraph::default()
            .aligned(Alignment::Center)
            .string(data.repository)
            .styled(Style::sized(SUBTITLE_FOOT_FONT_SIZE))
            .add_to(&mut doc);
        elements::Break::new(4).add_to(&mut doc);
        Paragraph::default()
            .aligned(Alignment::Center)
            .string("Evidencia I+D+I")
            .add_to(&mut doc);
        // elements::Image::from_dynamic_image(from);
        if let Err(err) = doc.render_to_file(output) {
            let boxed: Box<dyn std::error::Error> = Box::new(err);
            return Err(boxed);
        }
    }
    Ok(())
}
