use crossterm::style::{ContentStyle, Stylize};
use dioxus::prelude::*;
use sharp_tui::components::Input;
use sharp_tui::components::Separator;
use sharp_tui::components::{Button, Scrolling};
use sharp_tui::components::{
    ProgressBar, ProgressBarCharset, SeparatorCharset, SeparatorDirection, Spinner, SpinnerCharset,
};
use sharp_tui::components::{Select, SelectOption};
use sharp_tui::props::*;
use sharp_tui::{Block, Props, launch, no};
use sharp_tui::{Text, yoga};
use std::time::Duration;
use yoga::StyleUnit;

fn app() -> Element {
    let mut count = use_signal(|| 0u8);
    let mut input_value = use_signal(|| String::from(""));
    let mut scroll = use_signal(|| XYProps::<u16>::both(0));

    let mut is_masked = use_signal(|| false);
    let mut is_active = use_signal(|| false);

    rsx! {
        Scrolling {
            scroll: scroll(),
            on_scroll: move |offset| scroll.set(offset),
            Block {
                gap: XYProps::both(1),
                flex: Flex::new(no!(), no!(), no!(), no!(), FlexDirection::new(yoga::FlexDirection::Column)),
                ProgressBar {
                    percent: count(),
                    charset: ProgressBarCharset::diamond()
                }



                Button {
                    on_click: move || {
                        count.set(count().saturating_add(1).min(100))
                    },
                    active: false,

                    "Increment"
                }

                Button {
                    on_click: move || {
                        count.set(count().saturating_sub(1))
                    },
                    active: true,

                    "Decrement"

                }
            }
            Separator {
                charset: SeparatorCharset::box_char(),
                style: ContentStyle::default().dark_grey(),
                margin: Margin::new(Sides::all(2)),
                dir: SeparatorDirection::Horizontal,
            }
            Block {
                Input {
                    active: is_active(),
                    value: input_value(),
                    on_change: move |x: Box<str>| {
                        input_value.set(x.into());
                    },
                    placeholder: "Type something fun".to_string(),
                    mask_character: is_masked().then_some('*')
                }
                Button {
                    on_click: move || is_masked.set(!is_masked()),
                    active: false,

                    "Toggle mask"
                }
                Button {
                    on_click: move || is_active.set(!is_active()),
                    active: false,

                    "Toggle active"
                }
            }
            Block {
                Spinner {
                    charset: SpinnerCharset::dots()
                }
            }
            Block {
                Select {
                    active: true,
                    height: Height::from(
                        SizeClamp::new(
                            no!(),
                            SizeUnit::new(StyleUnit::Point(5u16.into())),
                            no!()
                        )
                    ),
                    on_enter: move |index: u32| {
                        input_value.set(format!("selected {index}"));
                    },
                    for i in 0..100u32 {
                        SelectOption {
                            key: "{i}",
                            index: i,
                            "Option {i}"
                        }
                    }
                }
            }
            Text {
                style: ContentStyle::new().red().bold(),
                "Funny how i am "
                Text {
                    style: ContentStyle::new().green(),
                    "green"
                }
                " and your not"
            }
        }

    }
}

fn main() -> std::io::Result<()> {
    launch(
        app,
        Props {
            ctrl_c: true,
            event_poll_speed: Duration::from_millis(10),
        },
    )
}
