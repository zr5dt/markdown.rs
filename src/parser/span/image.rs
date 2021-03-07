use parser::Span;
use parser::Span::Image;
use parser::ObjectSize;
use regex::Regex;

pub fn parse_image(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref IMAGE: Regex =
            Regex::new("^!\\[(?P<text>.*?)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?(?:\\s=(?P<size>[0-9xX%]*))?\\)")
                .unwrap();
    }

    if IMAGE.is_match(text) {
        let caps = IMAGE.captures(text).unwrap();
        let text = if let Some(mat) = caps.name("text") {
            mat.as_str().to_owned()
        } else {
            "".to_owned()
        };
        let url = if let Some(mat) = caps.name("url") {
            mat.as_str().to_owned()
        } else {
            "".to_owned()
        };
        let title = if let Some(mat) = caps.name("title") {
            Some(mat.as_str().to_owned())
        } else {
            None
        };
        let size = if let Some(mat) = caps.name("size") {
            ObjectSize::from_text(mat.as_str())
        } else {
            None
        };
        let len = caps.get(0).unwrap().end();
        return Some((Image(text, url, title, size), len));
    }
    None
}

#[test]
fn finds_image() {
    assert_eq!(
        parse_image("![an example](example.com) test"),
        Some((
            Image("an example".to_owned(), "example.com".to_owned(), None, None),
            26
        ))
    );

    assert_eq!(
        parse_image("![](example.com) test"),
        Some((Image("".to_owned(), "example.com".to_owned(), None, None), 16))
    );

    assert_eq!(
        parse_image("![an example]() test"),
        Some((Image("an example".to_owned(), "".to_owned(), None, None), 15))
    );

    assert_eq!(
        parse_image("![]() test"),
        Some((Image("".to_owned(), "".to_owned(), None, None), 5))
    );

    assert_eq!(
        parse_image("![an example](example.com \"Title\") test"),
        Some((
            Image(
                "an example".to_owned(),
                "example.com".to_owned(),
                Some("Title".to_owned()),
                None
            ),
            34
        ))
    );

    assert_eq!(
        parse_image("![an example](example.com) test [a link](example.com)"),
        Some((
            Image("an example".to_owned(), "example.com".to_owned(), None, None),
            26
        ))
    );

    // Check that we don't falsely trigger the size attribute
    assert_eq!(parse_image("![an example](http://host.com/filepath.jpg) =111x222"),
        Some((
            Image("an example".to_owned(), "http://host.com/filepath.jpg".to_owned(), None, None),
            43
        ))
    );
}

#[test]
fn no_false_positives() {
    assert_eq!(parse_image("![()] testing things test"), None);
    assert_eq!(parse_image("!()[] testing things test"), None);
}

#[test]
fn no_early_matching() {
    assert_eq!(parse_image("were ![an example](example.com) test"), None);
}

#[test]
fn with_size() {
    assert_eq!(parse_image("![my image](image.jpg =111x222) blah"),
        Some((
            Image("my image".to_owned(), "image.jpg".to_owned(), None,
                Some(ObjectSize{width: Some("111".to_string()), height: Some("222".to_string())})
            ),
            31
        ))
    );

    assert_eq!(parse_image("![my image](image.jpg =111x) blah"),
        Some((
            Image("my image".to_owned(), "image.jpg".to_owned(), None,
                Some(ObjectSize{width: Some("111".to_string()), height: None})
            ),
            28
        ))
    );

    assert_eq!(parse_image("![my image](image.jpg =x222) blah"),
        Some((
            Image("my image".to_owned(), "image.jpg".to_owned(), None,
                Some(ObjectSize{width: None, height: Some("222".to_string())})
            ),
            28
        ))
    );

    // With empty size
    assert_eq!(parse_image("![my image](image.jpg =) blah"),
        Some((
            Image("my image".to_owned(), "image.jpg".to_owned(), None, None),
            24
        ))
    );

    // With leading empty size and a trailing title
    assert_eq!(parse_image("![my image](image.jpg = \"silly title\") blah"),
        Some((
            Image("my image".to_owned(), "image.jpg =".to_owned(),
                Some("silly title".to_owned()),
                None
            ),
            38
        ))
    );

    // With trailing empty size and leading title
    assert_eq!(parse_image("![my image](image.jpg \"silly title\" =) blah"),
        Some((
            Image("my image".to_owned(), "image.jpg".to_owned(),
                Some("silly title".to_owned()),
                None
            ),
            38
        ))
    );

    // With trailing size and leading title
    assert_eq!(parse_image("![my image](image.jpg \"silly title\" =x222) blah"),
        Some((
            Image("my image".to_owned(), "image.jpg".to_owned(),
                Some("silly title".to_owned()),
                Some(ObjectSize{width: None, height: Some("222".to_string())})
            ),
            42
        ))
    );

    // Size property in wrong place
    assert_eq!(parse_image("![my image](image.jpg =x222 \"silly title\") blah"),
        Some((
            Image("my image".to_owned(), "image.jpg =x222".to_owned(),
                Some("silly title".to_owned()),
                None
            ),
            42
        ))
    );

}
