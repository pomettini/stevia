extern crate epub_builder;

use self::epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use regex::Regex;

use crate::reader::*;

#[derive(Default)]
pub struct EpubWriter {}

impl EpubWriter {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: Handle errors
    pub fn generate(&self) -> Option<Vec<u8>> {
        let image = "Not really a PNG image";
        let css = "";

        let title = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
	<title>Book title</title>
	<meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type" />
	<link href="stylesheet.css" rel="stylesheet" type="text/css" />
</head>
<body>
	<h1>Book title</h1>
	<p>by John Doe</p>
</body>
</html>"#;

        let cover = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?><html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Cover</title>
    <meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type"/>
    <style title="override_css" type="text/css">
        @page { padding: 0pt; margin: 0pt }
        body { text-align: center; padding:0pt; margin: 0pt; }
    </style>
</head>
<body>
    <div>
        <svg xmlns="http://www.w3.org/2000/svg" height="100%" version="1.1" width="100%" xmlns:xlink="http://www.w3.org/1999/xlink">
            <image xlink:href="cover.jpg"/>
        </svg>
    </div>
</body>
</html>"#;

        let page = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?><html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Page</title>
    <meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type"/>
    <link href="stylesheet.css" rel="stylesheet" type="text/css"/>
</head>
<body>
	<p>Hello world</p>
    <p>Ciao mondo</p>
    <ul>
        <li>First choice</li>
        <li>Second choice</li>
    </ul>
</body>
</html>"#;

        let mut epub: Vec<u8> = Vec::new();

        EpubBuilder::new(ZipLibrary::new().unwrap())
            .unwrap()
            .metadata("author", "Joan Doe")
            .unwrap()
            .metadata("title", "Dummy Book")
            .unwrap()
            .stylesheet(css.as_bytes())
            .unwrap()
            .add_cover_image("cover.jpg", image.as_bytes(), "image/png")
            .unwrap()
            .add_content(
                EpubContent::new("cover.xhtml", cover.as_bytes())
                    .title("Cover")
                    .reftype(ReferenceType::Cover),
            )
            .unwrap()
            .add_content(
                EpubContent::new("title.xhtml", title.as_bytes())
                    .title("Title")
                    .reftype(ReferenceType::TitlePage),
            )
            .unwrap()
            .add_content(
                EpubContent::new("chapter_1.xhtml", page.as_bytes())
                    .title("Chapter 1")
                    .reftype(ReferenceType::Text),
            )
            .unwrap()
            .add_content(
                EpubContent::new("chapter_2.xhtml", page.as_bytes())
                    .title("Chapter 2")
                    .reftype(ReferenceType::Text),
            )
            .unwrap()
            .generate(&mut epub)
            .unwrap();

        Some(epub)
    }
}
