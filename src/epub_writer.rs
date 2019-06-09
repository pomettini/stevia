extern crate epub_builder;

use self::epub_builder::{EpubBuilder, EpubContent, ReferenceType, Result, TocElement, ZipLibrary};
use regex::Regex;
use std::collections::*;

use std::io;
use std::io::Write;

use crate::reader::*;

#[derive(Default)]
pub struct EpubWriter {}

impl EpubWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) -> Result<()> {
        let dummy_content = "Dummy content. This should be valid XHTML if you want a valid EPUB!";
        let dummy_image = "Not really a PNG image";
        let dummy_css = "body { background-color: pink }";

        let mut epub: Vec<u8> = Vec::new();

        EpubBuilder::new(ZipLibrary::new()?)?
            .metadata("author", "Joan Doe")?
            .metadata("title", "Dummy Book")?
            .stylesheet(dummy_css.as_bytes())?
            .add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?
            .add_content(
                EpubContent::new("cover.xhtml", dummy_content.as_bytes())
                    .title("Cover")
                    .reftype(ReferenceType::Cover),
            )?
            .add_content(
                EpubContent::new("title.xhtml", dummy_content.as_bytes())
                    .title("Title")
                    .reftype(ReferenceType::TitlePage),
            )?
            .add_content(
                EpubContent::new("chapter_1.xhtml", dummy_content.as_bytes())
                    .title("Chapter 1")
                    .reftype(ReferenceType::Text),
            )?
            .generate(&mut epub)?;

        Ok(())
    }
}
