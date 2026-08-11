use crate::{Block, Text};
use crossterm::style::ContentStyle;
use dioxus::prelude::*;
use dioxus_sdk_time::use_interval;
use std::time::Duration;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpinnerCharset {
    interval: Duration,
    frames: Box<[Box<str>]>,
}

macro_rules! define_unstyled_spinner {
    ($interval:expr, $($frame:literal),+) => {
        SpinnerCharset {
            interval: Duration::from_millis($interval),
            frames: Box::new([$(
              Box::from($frame)
            ),+]),
        }
	};
}
// Source: github.com/sindresorhus/cli-spinners/blob/main/spinners.json
impl SpinnerCharset {
    pub fn dots() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏")
    }
    pub fn dots2() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷")
    }
    pub fn dots3() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓")
    }
    pub fn dots4() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆"
        )
    }
    pub fn dots5() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠋", "⠙", "⠚", "⠒", "⠂", "⠂", "⠒", "⠲", "⠴", "⠦", "⠖", "⠒", "⠐", "⠐", "⠒", "⠓", "⠋"
        )
    }
    pub fn dots6() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠁", "⠉", "⠙", "⠚", "⠒", "⠂", "⠂", "⠒", "⠲", "⠴", "⠤", "⠄", "⠄", "⠤", "⠴", "⠲",
            "⠒", "⠂", "⠂", "⠒", "⠚", "⠙", "⠉", "⠁"
        )
    }
    pub fn dots7() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠈", "⠉", "⠋", "⠓", "⠒", "⠐", "⠐", "⠒", "⠖", "⠦", "⠤", "⠠", "⠠", "⠤", "⠦", "⠖",
            "⠒", "⠐", "⠐", "⠒", "⠓", "⠋", "⠉", "⠈"
        )
    }
    pub fn dots8() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠁", "⠁", "⠉", "⠙", "⠚", "⠒", "⠂", "⠂", "⠒", "⠲", "⠴", "⠤", "⠄", "⠄", "⠤", "⠠",
            "⠠", "⠤", "⠦", "⠖", "⠒", "⠐", "⠐", "⠒", "⠓", "⠋", "⠉", "⠈", "⠈"
        )
    }
    pub fn dots9() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⢹", "⢺", "⢼", "⣸", "⣇", "⡧", "⡗", "⡏")
    }
    pub fn dots10() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠")
    }
    pub fn dots11() -> SpinnerCharset {
        define_unstyled_spinner!(100, "⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈")
    }
    pub fn dots12() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁",
            "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘",
            "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩",
            "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀"
        )
    }
    pub fn dots13() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⣼", "⣹", "⢻", "⠿", "⡟", "⣏", "⣧", "⣶")
    }
    pub fn dots14() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠉⠉", "⠈⠙", "⠀⠹", "⠀⢸", "⠀⣰", "⢀⣠", "⣀⣀", "⣄⡀", "⣆⠀", "⡇⠀", "⠏⠀", "⠋⠁"
        )
    }
    pub fn dots8bit() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠀", "⠁", "⠂", "⠃", "⠄", "⠅", "⠆", "⠇", "⡀", "⡁", "⡂", "⡃", "⡄", "⡅", "⡆", "⡇",
            "⠈", "⠉", "⠊", "⠋", "⠌", "⠍", "⠎", "⠏", "⡈", "⡉", "⡊", "⡋", "⡌", "⡍", "⡎", "⡏", "⠐",
            "⠑", "⠒", "⠓", "⠔", "⠕", "⠖", "⠗", "⡐", "⡑", "⡒", "⡓", "⡔", "⡕", "⡖", "⡗", "⠘", "⠙",
            "⠚", "⠛", "⠜", "⠝", "⠞", "⠟", "⡘", "⡙", "⡚", "⡛", "⡜", "⡝", "⡞", "⡟", "⠠", "⠡", "⠢",
            "⠣", "⠤", "⠥", "⠦", "⠧", "⡠", "⡡", "⡢", "⡣", "⡤", "⡥", "⡦", "⡧", "⠨", "⠩", "⠪", "⠫",
            "⠬", "⠭", "⠮", "⠯", "⡨", "⡩", "⡪", "⡫", "⡬", "⡭", "⡮", "⡯", "⠰", "⠱", "⠲", "⠳", "⠴",
            "⠵", "⠶", "⠷", "⡰", "⡱", "⡲", "⡳", "⡴", "⡵", "⡶", "⡷", "⠸", "⠹", "⠺", "⠻", "⠼", "⠽",
            "⠾", "⠿", "⡸", "⡹", "⡺", "⡻", "⡼", "⡽", "⡾", "⡿", "⢀", "⢁", "⢂", "⢃", "⢄", "⢅", "⢆",
            "⢇", "⣀", "⣁", "⣂", "⣃", "⣄", "⣅", "⣆", "⣇", "⢈", "⢉", "⢊", "⢋", "⢌", "⢍", "⢎", "⢏",
            "⣈", "⣉", "⣊", "⣋", "⣌", "⣍", "⣎", "⣏", "⢐", "⢑", "⢒", "⢓", "⢔", "⢕", "⢖", "⢗", "⣐",
            "⣑", "⣒", "⣓", "⣔", "⣕", "⣖", "⣗", "⢘", "⢙", "⢚", "⢛", "⢜", "⢝", "⢞", "⢟", "⣘", "⣙",
            "⣚", "⣛", "⣜", "⣝", "⣞", "⣟", "⢠", "⢡", "⢢", "⢣", "⢤", "⢥", "⢦", "⢧", "⣠", "⣡", "⣢",
            "⣣", "⣤", "⣥", "⣦", "⣧", "⢨", "⢩", "⢪", "⢫", "⢬", "⢭", "⢮", "⢯", "⣨", "⣩", "⣪", "⣫",
            "⣬", "⣭", "⣮", "⣯", "⢰", "⢱", "⢲", "⢳", "⢴", "⢵", "⢶", "⢷", "⣰", "⣱", "⣲", "⣳", "⣴",
            "⣵", "⣶", "⣷", "⢸", "⢹", "⢺", "⢻", "⢼", "⢽", "⢾", "⢿", "⣸", "⣹", "⣺", "⣻", "⣼", "⣽",
            "⣾", "⣿"
        )
    }
    pub fn dots_circle() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⢎ ", "⠎⠁", "⠊⠑", "⠈⠱", " ⡱", "⢀⡰", "⢄⡠", "⢆⡀")
    }
    pub fn sand() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦",
            "⣮", "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑",
            "⠡", "⢁"
        )
    }
    pub fn line() -> SpinnerCharset {
        define_unstyled_spinner!(130, "-", "\\", "|", "/")
    }
    pub fn line2() -> SpinnerCharset {
        define_unstyled_spinner!(100, "⠂", "-", "–", "—", "–", "-")
    }
    pub fn rolling_line() -> SpinnerCharset {
        define_unstyled_spinner!(80, "/  ", " - ", " \\ ", "  |", "  |", " \\ ", " - ", "/  ")
    }
    pub fn pipe() -> SpinnerCharset {
        define_unstyled_spinner!(100, "┤", "┘", "┴", "└", "├", "┌", "┬", "┐")
    }
    pub fn simple_dots() -> SpinnerCharset {
        define_unstyled_spinner!(400, ".  ", ".. ", "...", "   ")
    }
    pub fn simple_dots_scrolling() -> SpinnerCharset {
        define_unstyled_spinner!(200, ".  ", ".. ", "...", " ..", "  .", "   ")
    }
    pub fn star() -> SpinnerCharset {
        define_unstyled_spinner!(70, "✶", "✸", "✹", "✺", "✹", "✷")
    }
    pub fn star2() -> SpinnerCharset {
        define_unstyled_spinner!(80, "+", "x", "*")
    }
    pub fn flip() -> SpinnerCharset {
        define_unstyled_spinner!(
            70, "_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"
        )
    }
    pub fn hamburger() -> SpinnerCharset {
        define_unstyled_spinner!(100, "☱", "☲", "☴")
    }
    pub fn grow_vertical() -> SpinnerCharset {
        define_unstyled_spinner!(120, "▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃")
    }
    pub fn grow_horizontal() -> SpinnerCharset {
        define_unstyled_spinner!(
            120, "▏", "▎", "▍", "▌", "▋", "▊", "▉", "▊", "▋", "▌", "▍", "▎"
        )
    }
    pub fn balloon() -> SpinnerCharset {
        define_unstyled_spinner!(140, " ", ".", "o", "O", "@", "*", " ")
    }
    pub fn balloon2() -> SpinnerCharset {
        define_unstyled_spinner!(120, ".", "o", "O", "°", "O", "o", ".")
    }
    pub fn noise() -> SpinnerCharset {
        define_unstyled_spinner!(100, "▓", "▒", "░")
    }
    pub fn bounce() -> SpinnerCharset {
        define_unstyled_spinner!(120, "⠁", "⠂", "⠄", "⠂")
    }
    pub fn box_bounce() -> SpinnerCharset {
        define_unstyled_spinner!(120, "▖", "▘", "▝", "▗")
    }
    pub fn box_bounce2() -> SpinnerCharset {
        define_unstyled_spinner!(100, "▌", "▀", "▐", "▄")
    }
    pub fn triangle() -> SpinnerCharset {
        define_unstyled_spinner!(50, "◢", "◣", "◤", "◥")
    }
    pub fn binary() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "010010", "001100", "100101", "111010", "111101", "010111", "101011", "111000",
            "110011", "110101"
        )
    }
    pub fn arc() -> SpinnerCharset {
        define_unstyled_spinner!(100, "◜", "◠", "◝", "◞", "◡", "◟")
    }
    pub fn circle() -> SpinnerCharset {
        define_unstyled_spinner!(120, "◡", "⊙", "◠")
    }
    pub fn square_corners() -> SpinnerCharset {
        define_unstyled_spinner!(180, "◰", "◳", "◲", "◱")
    }
    pub fn circle_quarters() -> SpinnerCharset {
        define_unstyled_spinner!(120, "◴", "◷", "◶", "◵")
    }
    pub fn circle_halves() -> SpinnerCharset {
        define_unstyled_spinner!(50, "◐", "◓", "◑", "◒")
    }
    pub fn squish() -> SpinnerCharset {
        define_unstyled_spinner!(100, "╫", "╪")
    }
    pub fn toggle() -> SpinnerCharset {
        define_unstyled_spinner!(250, "⊶", "⊷")
    }
    pub fn toggle2() -> SpinnerCharset {
        define_unstyled_spinner!(80, "▫", "▪")
    }
    pub fn toggle3() -> SpinnerCharset {
        define_unstyled_spinner!(120, "□", "■")
    }
    pub fn toggle4() -> SpinnerCharset {
        define_unstyled_spinner!(100, "■", "□", "▪", "▫")
    }
    pub fn toggle5() -> SpinnerCharset {
        define_unstyled_spinner!(100, "▮", "▯")
    }
    pub fn toggle6() -> SpinnerCharset {
        define_unstyled_spinner!(300, "ဝ", "၀")
    }
    pub fn toggle7() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⦾", "⦿")
    }
    pub fn toggle8() -> SpinnerCharset {
        define_unstyled_spinner!(100, "◍", "◌")
    }
    pub fn toggle9() -> SpinnerCharset {
        define_unstyled_spinner!(100, "◉", "◎")
    }
    pub fn toggle10() -> SpinnerCharset {
        define_unstyled_spinner!(100, "㊂", "㊀", "㊁")
    }
    pub fn toggle11() -> SpinnerCharset {
        define_unstyled_spinner!(50, "⧇", "⧆")
    }
    pub fn toggle12() -> SpinnerCharset {
        define_unstyled_spinner!(120, "☗", "☖")
    }
    pub fn toggle13() -> SpinnerCharset {
        define_unstyled_spinner!(80, "=", "*", "-")
    }
    pub fn arrow() -> SpinnerCharset {
        define_unstyled_spinner!(100, "←", "↖", "↑", "↗", "→", "↘", "↓", "↙")
    }
    pub fn arrow2() -> SpinnerCharset {
        define_unstyled_spinner!(80, "⬆️ ", "↗️ ", "➡️ ", "↘️ ", "⬇️ ", "↙️ ", "⬅️ ", "↖️ ")
    }
    pub fn arrow3() -> SpinnerCharset {
        define_unstyled_spinner!(120, "▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸")
    }
    pub fn bouncing_bar() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]",
            "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]"
        )
    }
    pub fn bouncing_ball() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            "( ●    )",
            "(  ●   )",
            "(   ●  )",
            "(    ● )",
            "(     ●)",
            "(    ● )",
            "(   ●  )",
            "(  ●   )",
            "( ●    )",
            "(●     )"
        )
    }
    pub fn smiley() -> SpinnerCharset {
        define_unstyled_spinner!(200, "😄 ", "😝 ")
    }
    pub fn monkey() -> SpinnerCharset {
        define_unstyled_spinner!(300, "🙈 ", "🙈 ", "🙉 ", "🙊 ")
    }
    pub fn hearts() -> SpinnerCharset {
        define_unstyled_spinner!(100, "💛 ", "💙 ", "💜 ", "💚 ", "💗 ")
    }
    pub fn clock() -> SpinnerCharset {
        define_unstyled_spinner!(
            100, "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 "
        )
    }
    pub fn earth() -> SpinnerCharset {
        define_unstyled_spinner!(180, "🌍 ", "🌎 ", "🌏 ")
    }
    pub fn material() -> SpinnerCharset {
        define_unstyled_spinner!(
            17,
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "██████████▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "█████████████▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁██████████████▁▁▁▁",
            "▁▁▁██████████████▁▁▁",
            "▁▁▁▁█████████████▁▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁▁█████████████▁▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁▁███████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁▁█████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁"
        )
    }
    pub fn moon() -> SpinnerCharset {
        define_unstyled_spinner!(80, "🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 ")
    }
    pub fn runner() -> SpinnerCharset {
        define_unstyled_spinner!(140, "🚶 ", "🏃 ")
    }
    pub fn pong() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            "▐⠂       ▌",
            "▐⠈       ▌",
            "▐ ⠂      ▌",
            "▐ ⠠      ▌",
            "▐  ⡀     ▌",
            "▐  ⠠     ▌",
            "▐   ⠂    ▌",
            "▐   ⠈    ▌",
            "▐    ⠂   ▌",
            "▐    ⠠   ▌",
            "▐     ⡀  ▌",
            "▐     ⠠  ▌",
            "▐      ⠂ ▌",
            "▐      ⠈ ▌",
            "▐       ⠂▌",
            "▐       ⠠▌",
            "▐       ⡀▌",
            "▐      ⠠ ▌",
            "▐      ⠂ ▌",
            "▐     ⠈  ▌",
            "▐     ⠂  ▌",
            "▐    ⠠   ▌",
            "▐    ⡀   ▌",
            "▐   ⠠    ▌",
            "▐   ⠂    ▌",
            "▐  ⠈     ▌",
            "▐  ⠂     ▌",
            "▐ ⠠      ▌",
            "▐ ⡀      ▌",
            "▐⠠       ▌"
        )
    }
    pub fn shark() -> SpinnerCharset {
        define_unstyled_spinner!(
            120,
            "▐|\\____________▌",
            "▐_|\\___________▌",
            "▐__|\\__________▌",
            "▐___|\\_________▌",
            "▐____|\\________▌",
            "▐_____|\\_______▌",
            "▐______|\\______▌",
            "▐_______|\\_____▌",
            "▐________|\\____▌",
            "▐_________|\\___▌",
            "▐__________|\\__▌",
            "▐___________|\\_▌",
            "▐____________|\\▌",
            "▐____________/|▌",
            "▐___________/|_▌",
            "▐__________/|__▌",
            "▐_________/|___▌",
            "▐________/|____▌",
            "▐_______/|_____▌",
            "▐______/|______▌",
            "▐_____/|_______▌",
            "▐____/|________▌",
            "▐___/|_________▌",
            "▐__/|__________▌",
            "▐_/|___________▌",
            "▐/|____________▌"
        )
    }
    pub fn dqpb() -> SpinnerCharset {
        define_unstyled_spinner!(100, "d", "q", "p", "b")
    }
    pub fn weather() -> SpinnerCharset {
        define_unstyled_spinner!(
            100, "☀️ ", "☀️ ", "☀️ ", "🌤 ", "⛅️ ", "🌥 ", "☁️ ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ",
            "⛈ ", "🌨 ", "🌧 ", "🌨 ", "☁️ ", "🌥 ", "⛅️ ", "🌤 ", "☀️ ", "☀️ "
        )
    }
    pub fn christmas() -> SpinnerCharset {
        define_unstyled_spinner!(400, "🌲", "🎄")
    }
    pub fn grenade() -> SpinnerCharset {
        define_unstyled_spinner!(
            80, "،  ", "′  ", " ´ ", " ‾ ", "  ⸌", "  ⸊", "  |", "  ⁎", "  ⁕", " ෴ ", "  ⁓", "   ",
            "   ", "   "
        )
    }
    pub fn point() -> SpinnerCharset {
        define_unstyled_spinner!(125, "∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙")
    }
    pub fn layer() -> SpinnerCharset {
        define_unstyled_spinner!(150, "-", "=", "≡")
    }
    pub fn beta_wave() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            "ρββββββ",
            "βρβββββ",
            "ββρββββ",
            "βββρβββ",
            "ββββρββ",
            "βββββρβ",
            "ββββββρ"
        )
    }
    pub fn finger_dance() -> SpinnerCharset {
        define_unstyled_spinner!(160, "🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 ")
    }
    pub fn fist_bump() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
            "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
            "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
            "\u{3000}🤜\u{3000}\u{3000}🤛\u{3000} ",
            "\u{3000}\u{3000}🤜🤛\u{3000}\u{3000} ",
            "\u{3000}🤜✨🤛\u{3000}\u{3000} ",
            "🤜\u{3000}✨\u{3000}🤛\u{3000} "
        )
    }
    pub fn soccer_header() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            " 🧑⚽️       🧑 ",
            "🧑  ⚽️      🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑      ⚽️  🧑 ",
            "🧑       ⚽️🧑  ",
            "🧑      ⚽️  🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑  ⚽️      🧑 "
        )
    }
    pub fn mindblown() -> SpinnerCharset {
        define_unstyled_spinner!(
            160,
            "😐 ",
            "😐 ",
            "😮 ",
            "😮 ",
            "😦 ",
            "😦 ",
            "😧 ",
            "😧 ",
            "🤯 ",
            "💥 ",
            "✨ ",
            "\u{3000} ",
            "\u{3000} ",
            "\u{3000} "
        )
    }
    pub fn speaker() -> SpinnerCharset {
        define_unstyled_spinner!(160, "🔈 ", "🔉 ", "🔊 ", "🔉 ")
    }
    pub fn orange_pulse() -> SpinnerCharset {
        define_unstyled_spinner!(100, "🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 ")
    }
    pub fn blue_pulse() -> SpinnerCharset {
        define_unstyled_spinner!(100, "🔹 ", "🔷 ", "🔵 ", "🔵 ", "🔷 ")
    }
    pub fn orange_blue_pulse() -> SpinnerCharset {
        define_unstyled_spinner!(
            100, "🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 ", "🔹 ", "🔷 ", "🔵 ", "🔵 ", "🔷 "
        )
    }
    pub fn time_travel() -> SpinnerCharset {
        define_unstyled_spinner!(
            100, "🕛 ", "🕚 ", "🕙 ", "🕘 ", "🕗 ", "🕖 ", "🕕 ", "🕔 ", "🕓 ", "🕒 ", "🕑 ", "🕐 "
        )
    }
    pub fn aesthetic() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            "▰▱▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▰",
            "▰▱▱▱▱▱▱"
        )
    }
    pub fn dwarf_fortress() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            " ██████£££  ",
            "☺██████£££  ",
            "☺██████£££  ",
            "☺▓█████£££  ",
            "☺▓█████£££  ",
            "☺▒█████£££  ",
            "☺▒█████£££  ",
            "☺░█████£££  ",
            "☺░█████£££  ",
            "☺ █████£££  ",
            " ☺█████£££  ",
            " ☺█████£££  ",
            " ☺▓████£££  ",
            " ☺▓████£££  ",
            " ☺▒████£££  ",
            " ☺▒████£££  ",
            " ☺░████£££  ",
            " ☺░████£££  ",
            " ☺ ████£££  ",
            "  ☺████£££  ",
            "  ☺████£££  ",
            "  ☺▓███£££  ",
            "  ☺▓███£££  ",
            "  ☺▒███£££  ",
            "  ☺▒███£££  ",
            "  ☺░███£££  ",
            "  ☺░███£££  ",
            "  ☺ ███£££  ",
            "   ☺███£££  ",
            "   ☺███£££  ",
            "   ☺▓██£££  ",
            "   ☺▓██£££  ",
            "   ☺▒██£££  ",
            "   ☺▒██£££  ",
            "   ☺░██£££  ",
            "   ☺░██£££  ",
            "   ☺ ██£££  ",
            "    ☺██£££  ",
            "    ☺██£££  ",
            "    ☺▓█£££  ",
            "    ☺▓█£££  ",
            "    ☺▒█£££  ",
            "    ☺▒█£££  ",
            "    ☺░█£££  ",
            "    ☺░█£££  ",
            "    ☺ █£££  ",
            "     ☺█£££  ",
            "     ☺█£££  ",
            "     ☺▓£££  ",
            "     ☺▓£££  ",
            "     ☺▒£££  ",
            "     ☺▒£££  ",
            "     ☺░£££  ",
            "     ☺░£££  ",
            "     ☺ £££  ",
            "      ☺£££  ",
            "      ☺£££  ",
            "      ☺▓££  ",
            "      ☺▓££  ",
            "      ☺▒££  ",
            "      ☺▒££  ",
            "      ☺░££  ",
            "      ☺░££  ",
            "      ☺ ££  ",
            "       ☺££  ",
            "       ☺££  ",
            "       ☺▓£  ",
            "       ☺▓£  ",
            "       ☺▒£  ",
            "       ☺▒£  ",
            "       ☺░£  ",
            "       ☺░£  ",
            "       ☺ £  ",
            "        ☺£  ",
            "        ☺£  ",
            "        ☺▓  ",
            "        ☺▓  ",
            "        ☺▒  ",
            "        ☺▒  ",
            "        ☺░  ",
            "        ☺░  ",
            "        ☺   ",
            "        ☺  &",
            "        ☺ ☼&",
            "       ☺ ☼ &",
            "       ☺☼  &",
            "      ☺☼  & ",
            "      ‼   & ",
            "     ☺   &  ",
            "    ‼    &  ",
            "   ☺    &   ",
            "  ‼     &   ",
            " ☺     &    ",
            "‼      &    ",
            "      &     ",
            "      &     ",
            "     &   ░  ",
            "     &   ▒  ",
            "    &    ▓  ",
            "    &    £  ",
            "   &    ░£  ",
            "   &    ▒£  ",
            "  &     ▓£  ",
            "  &     ££  ",
            " &     ░££  ",
            " &     ▒££  ",
            "&      ▓££  ",
            "&      £££  ",
            "      ░£££  ",
            "      ▒£££  ",
            "      ▓£££  ",
            "      █£££  ",
            "     ░█£££  ",
            "     ▒█£££  ",
            "     ▓█£££  ",
            "     ██£££  ",
            "    ░██£££  ",
            "    ▒██£££  ",
            "    ▓██£££  ",
            "    ███£££  ",
            "   ░███£££  ",
            "   ▒███£££  ",
            "   ▓███£££  ",
            "   ████£££  ",
            "  ░████£££  ",
            "  ▒████£££  ",
            "  ▓████£££  ",
            "  █████£££  ",
            " ░█████£££  ",
            " ▒█████£££  ",
            " ▓█████£££  ",
            " ██████£££  ",
            " ██████£££  "
        )
    }
    pub fn fish() -> SpinnerCharset {
        define_unstyled_spinner!(
            80,
            "~~~~~~~~~~~~~~~~~~~~",
            "> ~~~~~~~~~~~~~~~~~~",
            "º> ~~~~~~~~~~~~~~~~~",
            "(º> ~~~~~~~~~~~~~~~~",
            "((º> ~~~~~~~~~~~~~~~",
            "<((º> ~~~~~~~~~~~~~~",
            "><((º> ~~~~~~~~~~~~~",
            " ><((º> ~~~~~~~~~~~~",
            "~ ><((º> ~~~~~~~~~~~",
            "~~ <>((º> ~~~~~~~~~~",
            "~~~ ><((º> ~~~~~~~~~",
            "~~~~ <>((º> ~~~~~~~~",
            "~~~~~ ><((º> ~~~~~~~",
            "~~~~~~ <>((º> ~~~~~~",
            "~~~~~~~ ><((º> ~~~~~",
            "~~~~~~~~ <>((º> ~~~~",
            "~~~~~~~~~ ><((º> ~~~",
            "~~~~~~~~~~ <>((º> ~~",
            "~~~~~~~~~~~ ><((º> ~",
            "~~~~~~~~~~~~ <>((º> ",
            "~~~~~~~~~~~~~ ><((º>",
            "~~~~~~~~~~~~~~ <>((º",
            "~~~~~~~~~~~~~~~ ><((",
            "~~~~~~~~~~~~~~~~ <>(",
            "~~~~~~~~~~~~~~~~~ ><",
            "~~~~~~~~~~~~~~~~~~ <",
            "~~~~~~~~~~~~~~~~~~~~"
        )
    }
}

#[component]
pub fn Spinner(charset: SpinnerCharset, #[props(default)] style: ContentStyle) -> Element {
    let mut index = use_signal(|| 0u32);

    let frame_count = charset.frames.len() as u32;

    use_interval(charset.interval, move |()| {
        index.set((index() + 1) % frame_count);
    });

    let character = &*charset.frames[index() as usize];

    rsx! {
        Block {
            Text {
                style,

                {character}
            }
        }
    }
}
