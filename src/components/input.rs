use crate::{no, use_key_event, Block, Flex, FlexDirection, KeyEvent, Text, match_key_event};
use crossterm::event::{KeyCode};
use crossterm::style::{ContentStyle, Stylize};
use dioxus::prelude::*;
use yoga::FlexDirection as YGFlexDirection;

fn prev_word_boundary(chars: &[char], pos: usize) -> usize {
    let mut i = pos;
    while i > 0 && chars[i - 1].is_whitespace() {
        i -= 1;
    }
    while i > 0 && !chars[i - 1].is_whitespace() {
        i -= 1;
    }
    i
}

fn next_word_boundary(chars: &[char], pos: usize) -> usize {
    let len = chars.len();
    let mut i = pos;
    while i < len && chars[i].is_whitespace() {
        i += 1;
    }
    while i < len && !chars[i].is_whitespace() {
        i += 1;
    }
    i
}

/// Ordered `(start, end)` span of the current selection, or `None` when the anchor coincides
/// with the cursor (nothing actually selected).
fn selection_range(cursor: usize, anchor: Option<usize>) -> Option<(usize, usize)> {
    let anchor = anchor?;
    if anchor == cursor {
        return None;
    }
    Some((anchor.min(cursor), anchor.max(cursor)))
}

/// Splits `chars` into contiguous same-style runs — the cursor cell, any selected span, and
/// everything else — so each run can be painted as its own `Text` (a `Text` node only supports
/// one style for its whole content). A cursor sitting past the last character still gets a
/// (blank) cell to render on, since there's otherwise nowhere to draw it.
fn build_runs(
    chars: &[char],
    cursor: usize,
    selection: Option<(usize, usize)>,
    normal_style: ContentStyle,
    selection_style: ContentStyle,
    cursor_style: ContentStyle,
) -> Vec<(ContentStyle, String)> {
    let total = chars.len().max(cursor + 1);
    let mut runs: Vec<(ContentStyle, String)> = Vec::new();
    for i in 0..total {
        let ch = chars.get(i).copied().unwrap_or(' ');
        let style = if i == cursor {
            cursor_style
        } else if selection.is_some_and(|(start, end)| i >= start && i < end) {
            selection_style
        } else {
            normal_style
        };
        match runs.last_mut() {
            Some((last_style, text)) if *last_style == style => text.push(ch),
            _ => runs.push((style, ch.to_string())),
        }
    }
    runs
}

#[component]
pub fn Input(
    value: String,
    on_change: EventHandler<Box<str>>,
    #[props(default)] placeholder: String,
    #[props(default)] style: ContentStyle,
) -> Element {
    let char_count = value.chars().count();
    let cursor = use_signal(move || char_count);
    let anchor = use_signal(|| None::<usize>);

    let chars: Vec<char> = value.chars().collect();
    let len = chars.len();
    let handler_chars = chars.clone();

    // `use_key_event` stores its handler as `Rc<dyn Fn(KeyEvent)>`, so it can't be `FnMut` —
    // `Signal::set` needs `&mut self`, which an `Fn` closure can't give its captures, so writes
    // here go through `write_unchecked` (`&self`) instead.
    use_key_event(move |e: KeyEvent| {
        if !e.is_press() && !e.is_repeat() {
            return;
        }

        let pos = cursor.peek().min(len);
        let shift = match_key_event!(e, Shift);
        let ctrl = match_key_event!(e, Control);
        let alt = match_key_event!(e, Alt);

        let mov = |new_pos: usize| {
            if shift {
                if anchor.peek().is_none() {
                    *anchor.write_unchecked() = Some(pos);
                }
            } else {
                *anchor.write_unchecked() = None;
            }
            *cursor.write_unchecked() = new_pos;
        };

        match e.code {
            KeyCode::Char('a') if ctrl => {
                *anchor.write_unchecked() = Some(0);
                *cursor.write_unchecked() = len;
            }
            KeyCode::Left => mov(if ctrl {
                prev_word_boundary(&handler_chars, pos)
            } else {
                pos.saturating_sub(1)
            }),
            KeyCode::Right => mov(if ctrl {
                next_word_boundary(&handler_chars, pos)
            } else {
                (pos + 1).min(len)
            }),
            KeyCode::Home => mov(0),
            KeyCode::End => mov(len),
            KeyCode::Backspace => {
                // Read into a plain local first: `if let` keeps scrutinee temporaries alive for
                // the whole arm body, so matching directly on `*anchor.peek()` here would hold
                // its borrow guard open across the `write_unchecked` calls below and panic.
                let anchor_pos = *anchor.peek();
                if let Some((start, end)) = selection_range(pos, anchor_pos) {
                    let new_value: String =
                        handler_chars[..start].iter().chain(&handler_chars[end..]).collect();
                    *anchor.write_unchecked() = None;
                    *cursor.write_unchecked() = start;
                    on_change.call(Box::from(new_value.as_str()));
                } else if pos > 0 {
                    let start = if ctrl {
                        prev_word_boundary(&handler_chars, pos)
                    } else {
                        pos - 1
                    };
                    let new_value: String =
                        handler_chars[..start].iter().chain(&handler_chars[pos..]).collect();
                    *anchor.write_unchecked() = None;
                    *cursor.write_unchecked() = start;
                    on_change.call(Box::from(new_value.as_str()));
                }
            }
            KeyCode::Delete => {
                let anchor_pos = *anchor.peek();
                if let Some((start, end)) = selection_range(pos, anchor_pos) {
                    let new_value: String =
                        handler_chars[..start].iter().chain(&handler_chars[end..]).collect();
                    *anchor.write_unchecked() = None;
                    *cursor.write_unchecked() = start;
                    on_change.call(Box::from(new_value.as_str()));
                } else if pos < len {
                    let end = if ctrl {
                        next_word_boundary(&handler_chars, pos)
                    } else {
                        pos + 1
                    };
                    let new_value: String =
                        handler_chars[..pos].iter().chain(&handler_chars[end..]).collect();
                    on_change.call(Box::from(new_value.as_str()));
                }
            }
            KeyCode::Char(c) if !ctrl && !alt => {
                let (start, end) = selection_range(pos, *anchor.peek()).unwrap_or((pos, pos));
                let new_value: String = handler_chars[..start]
                    .iter()
                    .chain(std::iter::once(&c))
                    .chain(&handler_chars[end..])
                    .collect();
                *anchor.write_unchecked() = None;
                *cursor.write_unchecked() = start + 1;
                on_change.call(Box::from(new_value.as_str()));
            }
            _ => {}
        }
    });

    let selection = selection_range(cursor().min(len), anchor().map(|a| a.min(len)));
    let cursor_style = style.reverse();
    let selection_style = style.on_dark_grey();

    let runs = if chars.is_empty() {
        let placeholder_chars: Vec<char> = placeholder.chars().collect();
        build_runs(
            &placeholder_chars,
            0,
            None,
            ContentStyle::new().dark_grey(),
            selection_style,
            cursor_style,
        )
    } else {
        build_runs(
            &chars,
            cursor().min(len),
            selection,
            style,
            selection_style,
            cursor_style,
        )
    };

    rsx! {
        Block {
            flex: Flex::new(no!(), no!(), no!(), no!(), FlexDirection::new(YGFlexDirection::Row)),
            for (i, (run_style, text)) in runs.into_iter().enumerate() {
                Text {
                    key: "{i}",
                    style: run_style,
                    "{text}"
                }
            }
        }
    }
}