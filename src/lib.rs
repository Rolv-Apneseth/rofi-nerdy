use rofi_mode::{Action, Event};
use std::sync::Arc;
use tracing::error;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod icons;
mod utils;

use icons::get_icons;
use utils::{copy_to_clipboard, insert};

struct Icon<'a> {
    name: &'a str,
    char: &'a str,
}

struct Mode<'rofi> {
    entries: Arc<[Icon<'rofi>]>,
}

// ROFI MODE
impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "nerdy\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("nerd icon");

        tracing_subscriber::registry()
            .with(fmt::layer().without_time().with_line_number(true))
            .with(EnvFilter::from_default_env())
            .init();

        let entries = get_icons()
            .into_iter()
            .map(|[char, name]| Icon { name, char })
            .collect();

        Ok(Mode { entries })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        rofi_mode::format!("{} {}", entry.char, entry.name)
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        _input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match event {
            // User accepted an option from the list
            Event::Ok { alt, selected } => {
                let selected_entry = &self.entries[selected];

                // User selected entry with alternative binding, attempt to simulate
                // typing icon                if alt {
                if alt {
                    if let Err(e) = insert(selected_entry.char) {
                        error!("Error with typing out icon: {e:?}")
                    };
                // User selected entry regularly, copy the icon to the system clipboard
                } else if let Err(e) = copy_to_clipboard(selected_entry.char) {
                    error!("Error with copying icon to clipboard: {e:?}")
                }
            }

            // User cancelled selection i.e. pressed `Esc`
            Event::Cancel { selected: _ } => {}

            // All other events are unsupported
            _ => {
                error!("Unsupported input event: {event:?}")
            }
        }

        Action::Exit
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(self.entries[line].name)
    }

    fn entry_style(&self, _line: usize) -> rofi_mode::Style {
        rofi_mode::Style::default()
    }

    fn entry_attributes(&self, _line: usize) -> rofi_mode::Attributes {
        rofi_mode::Attributes::new()
    }

    fn completed(&self, line: usize) -> rofi_mode::String {
        self.entry_content(line)
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        rofi_mode::String::new()
    }
}

rofi_mode::export_mode!(Mode);
