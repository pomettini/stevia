extern crate epub_builder;
extern crate image;

use self::epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use self::image::*;

#[derive(Default)]
pub struct EpubWriter {
    pub title: String,
    pub author: String,
    pub cover_path: String,
}

impl EpubWriter {
    pub fn new(title: &str, author: &str, cover_path: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            cover_path: cover_path.to_string(),
        }
    }

    // TODO: Handle errors
    pub fn generate(&self) -> Option<Vec<u8>> {
        let image = image::open(&self.cover_path).unwrap();

        let mut jpg = Vec::new();
        image.write_to(&mut jpg, JPEG).unwrap();

        let css = "";

        let mut epub: Vec<u8> = Vec::new();

        EpubBuilder::new(ZipLibrary::new().unwrap())
            .unwrap()
            .metadata("author", self.author.clone())
            .unwrap()
            .metadata("title", self.title.clone())
            .unwrap()
            .stylesheet(css.as_bytes())
            .unwrap()
            .add_cover_image("cover.jpg", jpg.as_slice(), "image/jpg")
            .unwrap()
            .add_content(
                EpubContent::new("cover.xhtml", self.cover_builder(image.height(), image.width()).as_bytes())
                    .title("Cover")
                    .reftype(ReferenceType::Cover),
            )
            .unwrap()
            .add_content(
                EpubContent::new("title.xhtml", self.title_builder().as_bytes())
                    .title("Title")
                    .reftype(ReferenceType::TitlePage),
            )
            .unwrap()
            .add_content(
                EpubContent::new(
                    "chapter_1.xhtml",
                    self.page_builder("<p>Hello world</p>").as_bytes(),
                )
                .title("Chapter 1")
                .reftype(ReferenceType::Text),
            )
            .unwrap()
            .add_content(
                EpubContent::new(
                    "chapter_2.xhtml",
                    self.page_builder("<p>Ciao mondo</p>").as_bytes(),
                )
                .title("Chapter 2")
                .reftype(ReferenceType::Text),
            )
            .unwrap()
            .generate(&mut epub)
            .unwrap();

        Some(epub)
    }

    pub fn cover_builder(&self, height: u32, width: u32) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?><html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Cover</title>
    <meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type"/>
    <style title="override_css" type="text/css">
        @page {{ padding: 0pt; margin: 0pt }}
        body {{ text-align: center; padding:0pt; margin: 0pt; }}
    </style>
</head>
<body>
    <div>
        <svg xmlns="http://www.w3.org/2000/svg" height="100%" version="1.1" viewBox="0 0 {width} {height}" width="100%" xmlns:xlink="http://www.w3.org/1999/xlink">
            <image height="{height}" width="{width}" xlink:href="{filename}"/>
        </svg>
    </div>
</body>
</html>"#, height = height, width = width, filename = "cover.jpg")
    }

    pub fn title_builder(&self) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
	<title>{title}</title>
	<meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type" />
	<link href="stylesheet.css" rel="stylesheet" type="text/css" />
</head>
<body>
	<h1>{title}</h1>
	<p>{author}</p>
</body>
</html>"#,
            title = self.title,
            author = self.author
        )
    }

    pub fn page_builder(&self, content: &str) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?><html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Page</title>
    <meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type"/>
    <link href="stylesheet.css" rel="stylesheet" type="text/css"/>
</head>
<body>
    {}
</body>
</html>"#, content)
    }
}
