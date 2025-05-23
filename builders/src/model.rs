/// Data models for testing the application.
#[cfg(test)]
pub mod test {
    use twilight_model::channel::message::component::{
        ButtonStyle, SelectMenuOption, TextInputStyle,
    };
    use twilight_model::channel::message::Component;

    use crate::component::{ActionRowBuilder, ButtonBuilder, SelectMenuOptionBuilder};
    use crate::modal::TextInputBuilder;

    pub const CUSTOM_ID: &str = "custom_id";
    pub const TEXT: &str = "Label";

    /// Returns a [primary](ButtonStyle::Primary) button.
    pub fn button() -> Component {
        ButtonBuilder::new(CUSTOM_ID, ButtonStyle::Primary)
            .build()
            .expect("expected button to be valid")
    }

    /// Returns a [short](TextInputStyle::Short) text input.
    pub fn text_input() -> Component {
        TextInputBuilder::new(TEXT, CUSTOM_ID, TextInputStyle::Short)
            .build()
            .expect("expected text input to be valid")
    }

    /// Returns an [action row](Component::ActionRow) with a single [text input](Component::TextInput).
    pub fn modal_action_row() -> Component {
        ActionRowBuilder::new()
            .add_component(text_input())
            .build()
            .expect("expected action row to be valid")
    }

    pub fn select_menu_option() -> SelectMenuOption {
        SelectMenuOptionBuilder::new(TEXT, CUSTOM_ID).build()
    }
}
