use std::borrow::Cow;
use textwrap::fill;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum TextWrap {
    #[default]
    Wrap,
    Hard,
    TruncateStart,
    TruncateMiddle,
    Truncate,
    Cut
}

impl TextWrap {
    pub fn process<'a>(&self, text: &'a str, width: usize) -> Cow<'a, str> {
        if text.len() <= width || width == 0 {
            return Cow::Borrowed(text);
        }

        match self {
            TextWrap::Wrap => Cow::Owned(fill(text, width)),

            TextWrap::Hard => {
                if text.len() <= width {
                    return Cow::Borrowed(text);
                }
                let mut result = String::with_capacity(text.len() + (text.len() / width));
                for (i, ch) in text.chars().enumerate() {
                    if i > 0 && i % width == 0 {
                        result.push('\n');
                    }
                    result.push(ch);
                }
                Cow::Owned(result)
            }

            TextWrap::TruncateStart => {
                if width <= 3 {
                    return Cow::Borrowed(&text[text.len() - width..]);
                }
                let keep = width - 3;
                let start_idx = text.len() - keep;
                Cow::Owned(format!("...{}", &text[start_idx..]))
            }

            TextWrap::TruncateMiddle => {
                if width <= 3 {
                    return Cow::Borrowed(&text[..width]);
                }
                let keep = width - 3;
                let half = keep / 2;
                let extra = keep % 2;
                Cow::Owned(format!(
                    "{}...{}",
                    &text[..half + extra],
                    &text[text.len() - half..]
                ))
            }

            TextWrap::Truncate => {
                if width <= 3 {
                    return Cow::Borrowed(&text[..width]);
                }
                let keep = width - 3;
                Cow::Owned(format!("{}...", &text[..keep]))
            }

            TextWrap::Cut => {
                match text.char_indices().nth(width) {
                    None => Cow::Borrowed(text),
                    Some((byte_idx, _)) => Cow::Owned(text[..byte_idx].to_string()),
                }
            }
        }
    }
}
