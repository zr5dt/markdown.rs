mod block;
mod span;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrderedListType {
    Numeric,
    Lowercase,
    Uppercase,
    LowercaseRoman,
    UppercaseRoman,
}

impl OrderedListType {
    pub fn from_str(type_str: &str) -> OrderedListType {
        match type_str {
            "a" => OrderedListType::Lowercase,
            "A" => OrderedListType::Uppercase,
            "i" => OrderedListType::LowercaseRoman,
            "I" => OrderedListType::UppercaseRoman,
            _ => OrderedListType::Numeric,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            OrderedListType::Lowercase => "a",
            OrderedListType::Uppercase => "A",
            OrderedListType::LowercaseRoman => "i",
            OrderedListType::UppercaseRoman => "I",
            OrderedListType::Numeric => "1",
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    Header(Vec<Span>, usize),
    Paragraph(Vec<Span>),
    Blockquote(Vec<Block>),
    CodeBlock(Option<String>, String),
    /** A link reference with the fields: (id, url, [title]) **/
    LinkReference(String, String, Option<String>),
    OrderedList(Vec<ListItem>, OrderedListType),
    UnorderedList(Vec<ListItem>),
    Raw(String),
    Hr,
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ListItem {
    Simple(Vec<Span>),
    Paragraph(Vec<Block>),
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectSize {
    width: Option<String>,
    height: Option<String>,
}

impl ObjectSize {
    pub fn as_text(&self) -> String {
        match (self.width.as_ref(), self.height.as_ref()) {
            (Some(w), Some(h)) => format!("{}x{}", w, h),
            (Some(w), None) => format!("{}x", w),
            (None, Some(h)) => format!("x{}", h),
            (None, None) => "".to_string(),
        }
    }
    pub fn as_html(&self) -> String {
        match (self.width.as_ref(), self.height.as_ref()) {
            (Some(w), Some(h)) => format!("width=\"{}\" height=\"{}\"", w, h),
            (Some(w), None) => format!("width=\"{}\"", w),
            (None, Some(h)) => format!("height=\"{}\"", h),
            (None, None) => "".to_string(),
        }
    }
    pub fn from_text(text: &str) -> Option<ObjectSize> {
        if let Some(i) = text.find(|c: char| c == 'x' || c == 'X') {
            let w = text[0..i].trim();
            let h = text[i+1..].trim();
            if !w.is_empty() || !h.is_empty() {
                Some(ObjectSize {
                    width: if w.is_empty() {None} else {Some(w.to_string())},
                    height: if h.is_empty() {None} else {Some(h.to_string())},
                })
            }
            else {
                None
            }
        }
        else if !text.is_empty() {
            Some(ObjectSize {
                width: Some(text.to_string()),
                height: None,
            })
        }
        else {
            None
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Span {
    Break,
    Text(String),
    Code(String),
    Literal(char),
    Link(Vec<Span>, String, Option<String>),
    /**
     * A reference-style link with the fields: (content, url, raw)
     * The "raw" field is used internally for falling back to the original
     * markdown link if the corresponding reference is not found at render time.
     **/
    RefLink(Vec<Span>, String, String),
    Image(String, String, Option<String>, Option<ObjectSize>),

    Emphasis(Vec<Span>),
    Strong(Vec<Span>),
}

pub fn parse(md: &str) -> Vec<Block> {
    block::parse_blocks(md)
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::ObjectSize;

    #[test]
    fn text_to_object_size__full() {
        assert_eq!(ObjectSize::from_text("123x456"),
            Some(ObjectSize{width: Some("123".to_string()), height: Some("456".to_string())})
        );
        assert_eq!(ObjectSize::from_text("321X654"),
            Some(ObjectSize{width: Some("321".to_string()), height: Some("654".to_string())})
        );
        assert_eq!(ObjectSize::from_text("50%x80%"),
            Some(ObjectSize{width: Some("50%".to_string()), height: Some("80%".to_string())})
        );
    }

    #[test]
    fn text_to_object_size__half() {
        assert_eq!(ObjectSize::from_text("111x"),
            Some(ObjectSize{width: Some("111".to_string()), height: None})
        );
        assert_eq!(ObjectSize::from_text("x222"),
            Some(ObjectSize{width: None, height: Some("222".to_string())})
        );
        assert_eq!(ObjectSize::from_text("333"),
            Some(ObjectSize{width: Some("333".to_string()), height: None})
        );
        assert_eq!(ObjectSize::from_text("60%"),
            Some(ObjectSize{width: Some("60%".to_string()), height: None})
        );
        assert_eq!(ObjectSize::from_text("60%x"),
            Some(ObjectSize{width: Some("60%".to_string()), height: None})
        );
        assert_eq!(ObjectSize::from_text("x60%"),
            Some(ObjectSize{width: None, height: Some("60%".to_string())})
        );
    }

    #[test]
    fn text_to_object_size__null() {
        assert_eq!(ObjectSize::from_text(""), None);
        assert_eq!(ObjectSize::from_text("x"), None);
    }

    #[test]
    fn text_to_object_size__wierd() {
        assert_eq!(ObjectSize::from_text("1A3x123"),
            Some(ObjectSize{width: Some("1A3".to_string()), height: Some("123".to_string())})
        );
        assert_eq!(ObjectSize::from_text("3a"),
            Some(ObjectSize{width: Some("3a".to_string()), height: None})
        );
        assert_eq!(ObjectSize::from_text("zxq"),
            Some(ObjectSize{width: Some("z".to_string()), height: Some("q".to_string())})
        );
    }

    #[test]
    fn object_size_as_text() {
        assert_eq!(ObjectSize{width: None, height: None}.as_text(), "");
        assert_eq!(ObjectSize{width: Some("111".to_string()), height: None}.as_text(), "111x");
        assert_eq!(ObjectSize{width: None, height: Some("222".to_string())}.as_text(), "x222");
        assert_eq!(ObjectSize{width: Some("111".to_string()), height: Some("222".to_string())}.as_text(), "111x222");
    }

    #[test]
    fn object_size_as_html() {
        assert_eq!(ObjectSize{width: None, height: None}.as_html(), "");
        assert_eq!(ObjectSize{width: Some("111".to_string()), height: None}.as_html(), "width=\"111\"");
        assert_eq!(ObjectSize{width: None, height: Some("222".to_string())}.as_html(), "height=\"222\"");
        assert_eq!(ObjectSize{width: Some("111".to_string()), height: Some("222".to_string())}.as_html(), "width=\"111\" height=\"222\"");
    }

}
