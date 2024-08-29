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

	pub(crate) fn button() -> Component {
		ButtonBuilder::new(CUSTOM_ID, ButtonStyle::Primary).build()
	}

	pub(crate) fn text_input() -> Component {
		TextInputBuilder::new(TEXT, CUSTOM_ID, TextInputStyle::Short).build()
	}

	pub(crate) fn modal_action_row() -> Component {
		ActionRowBuilder::new().add_component(text_input()).build()
	}

	pub(crate) fn select_menu_option() -> SelectMenuOption {
		SelectMenuOptionBuilder::new(TEXT, CUSTOM_ID).build()
	}
}
