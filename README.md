<h1 align="center">Sharp-tui</h1>

<div align="center">
    <strong>Ink but for rust!</strong>
</div>
<br>
<div align="center">
    <a href="https://github.com/Bishops-exe/sharp-tui/blob/main/LICENSE" title="View license!"><img alt="License" src="https://img.shields.io/github/license/bishops-exe/sharp-tui?color=880088"/></a>
    <img alt="Static Badge" src="https://img.shields.io/badge/unsafe-forbidden-008800">
    <a href="https://github.com/Bishops-exe/sharp-tui/pulls" title="Open a pull request!"><img alt="Open a pull request!" src="https://img.shields.io/badge/PR-create one!-008800"></a>
    <a href="https://github.com/Bishops-exe/sharp-tui/issues/new" title="Report a bug!"><img alt="Report a bug!" src="https://img.shields.io/badge/Bug%3F-report_it!-880000"></a>
</div>
<br>

A terminal UI framework that renders [Dioxus](https://dioxuslabs.com/) components to the terminal, using [`yoga`](https://crates.io/crates/yoga) for flexbox layout and [`crossterm`](https://crates.io/crates/crossterm) for terminal I/O.

Write your TUI the same way you'd write a Dioxus app — components, `rsx!`, signals, hooks — and `sharp-tui` handles layout and painting to the alternate screen.

## Example

```rust
use dioxus::prelude::*;
use sharp_tui::components::{Button, Input};
use sharp_tui::props::*;
use sharp_tui::{Block, Props, Text, launch, no};

fn app() -> Element {
    let mut count = use_signal(|| 0u8);

    rsx! {
        Block {
            flex: Flex::new(no!(), no!(), no!(), no!(), FlexDirection::new(yoga::FlexDirection::Column)),

            Text { "Count: {count}" }

            Button {
                on_click: move || count.set(count() + 1),
                "Increment"
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    launch(app, Props::default())
}
```

See [`examples/example.rs`](examples/example.rs) for a fuller demo exercising every component. Run it with:

```sh
cargo run --example example
```

## Components

Each component lives behind its own feature flag (all enabled by default via the `all` feature):

| Component                 | Feature              | Description                                                                                                      |
|---------------------------|----------------------|------------------------------------------------------------------------------------------------------------------|
| `Block` / `Text`          | *(always available)* | The core layout/text primitives everything else is built from.                                                   |
| `Button`                  | `button`             | A clickable, focusable box with a `disabled` state.                                                              |
| `Input`                   | `input`              | A single-line text field with cursor movement and selection.                                                     |
| `ProgressBar`             | `progress-bar`       | A percentage-driven progress bar with swappable charsets.                                                        |
| `Scrolling`               | `scrolling`          | A scrollable viewport; scroll offset is controlled by the parent (`scroll` + `on_scroll`).                       |
| `Spinner`                 | `spinner`            | An animated loading indicator with swappable charsets.                                                           |
| `Separator`               | `separator`          | A horizontal or vertical rule that fills its container.                                                          |
| `Select` / `SelectOption` | `select`             | A keyboard-navigable list (Up/Down to move, Enter to submit) that auto-scrolls the highlighted option into view. |

## Layout

Layout props (`Margin`, `Padding`, `Width`, `Height`, `Flex`, `AlignItems`, `Border`, etc.) mirror Yoga's flexbox model and are applied directly to `Block`/`Text` nodes — see `src/props` for the full set.
