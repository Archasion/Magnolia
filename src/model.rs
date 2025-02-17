/// Data models for testing the application.
#[cfg(test)]
pub(crate) mod test {
    use twilight_model::channel::message::component::{
        ButtonStyle, SelectMenuOption, TextInputStyle,
    };
    use twilight_model::channel::message::Component;

    use crate::builders::component::{ActionRowBuilder, ButtonBuilder, SelectMenuOptionBuilder};
    use crate::builders::modal::TextInputBuilder;

    pub(crate) const CUSTOM_ID: &'static str = "custom_id";
    pub(crate) const TEXT: &'static str = "Label";

    /// Returns a [primary](ButtonStyle::Primary) button.
    pub(crate) fn button() -> Component {
        ButtonBuilder::new(CUSTOM_ID, ButtonStyle::Primary)
            .build()
            .expect("expected button to be valid")
    }

    /// Returns a [short](TextInputStyle::Short) text input.
    pub(crate) fn text_input() -> Component {
        TextInputBuilder::new(TEXT, CUSTOM_ID, TextInputStyle::Short)
            .build()
            .expect("expected text input to be valid")
    }

    /// Returns an [action row](Component::ActionRow) with a single [text input](Component::TextInput).
    pub(crate) fn modal_action_row() -> Component {
        ActionRowBuilder::new()
            .add_component(text_input())
            .build()
            .expect("expected action row to be valid")
    }

    pub(crate) fn select_menu_option() -> SelectMenuOption {
        SelectMenuOptionBuilder::new(TEXT, CUSTOM_ID).build()
    }
}
