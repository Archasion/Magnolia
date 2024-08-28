#![allow(dead_code)]

use twilight_model::channel::message::component::{
	ActionRow, Button, ButtonStyle, SelectMenu, SelectMenuOption, SelectMenuType,
};
use twilight_model::channel::message::{Component, ReactionType};
use twilight_model::channel::ChannelType;

pub struct ButtonBuilder {
	button: Component,
}

impl ButtonBuilder {
	pub fn new(style: ButtonStyle) -> Self {
		Self {
			button: Component::Button(Button {
				custom_id: None,
				disabled: false,
				emoji: None,
				label: None,
				url: None,
				style,
			}),
		}
	}

	pub fn custom_id(mut self, custom_id: &str) -> Self {
		if let Component::Button(button) = &mut self.button {
			button.custom_id = Some(custom_id.to_owned());
		}
		self
	}

	pub fn label(mut self, label: &str) -> Self {
		if let Component::Button(button) = &mut self.button {
			button.label = Some(label.to_owned());
		}
		self
	}

	pub fn disabled(mut self, disabled: bool) -> Self {
		if let Component::Button(button) = &mut self.button {
			button.disabled = disabled;
		}
		self
	}

	pub fn emoji(mut self, emoji: ReactionType) -> Self {
		if let Component::Button(button) = &mut self.button {
			button.emoji = Some(emoji);
		}
		self
	}

	pub fn url(mut self, url: &str) -> Self {
		if let Component::Button(button) = &mut self.button {
			button.url = Some(url.to_owned());
		}
		self
	}

	pub fn build(self) -> Component {
		if let Component::Button(button) = &self.button {
			if button.style == ButtonStyle::Link && button.url.is_none() {
				panic!("url is required for link buttons");
			}
			if button.style != ButtonStyle::Link && button.url.is_some() {
				panic!("url is only valid for link buttons");
			}
		}
		self.button
	}
}

pub struct ActionRowBuilder {
	components: Vec<Component>,
}

impl ActionRowBuilder {
	pub fn new() -> Self {
		Self {
			components: Vec::new(),
		}
	}

	pub fn set_components(mut self, components: Vec<Component>) -> Self {
		self.components = components;
		self
	}

	pub fn add_component(mut self, component: Component) -> Self {
		self.components.push(component);
		self
	}

	pub fn build(self) -> Component {
		if self.components.is_empty() {
			panic!("ActionRow must have at least one component");
		}
		Component::ActionRow(ActionRow {
			components: self.components,
		})
	}
}

pub struct SelectMenuBuilder {
	select_menu: Component,
}

impl SelectMenuBuilder {
	// Creates a text select menu by default
	pub fn new(custom_id: &str, kind: SelectMenuType) -> Self {
		Self {
			select_menu: Component::SelectMenu(SelectMenu {
				channel_types: None,
				custom_id: custom_id.to_owned(),
				default_values: None,
				disabled: false,
				options: None,
				placeholder: None,
				min_values: None,
				max_values: None,
				kind,
			}),
		}
	}

	pub fn disabled(mut self, disabled: bool) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			select_menu.disabled = disabled;
		}
		self
	}

	pub fn set_options(mut self, options: Vec<SelectMenuOption>) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			select_menu.options = Some(options);
		}
		self
	}

	pub fn add_option(mut self, option: SelectMenuOption) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			if select_menu.options.is_none() {
				select_menu.options = Some(Vec::new());
			}
			select_menu.options.as_mut().unwrap().push(option);
		}
		self
	}

	pub fn placeholder(mut self, placeholder: &str) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			select_menu.placeholder = Some(placeholder.to_owned());
		}
		self
	}

	pub fn min_values(mut self, min_values: u8) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			select_menu.min_values = Some(min_values);
		}
		self
	}

	pub fn max_values(mut self, max_values: u8) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			select_menu.max_values = Some(max_values);
		}
		self
	}

	pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
		if let Component::SelectMenu(select_menu) = &mut self.select_menu {
			select_menu.channel_types = Some(channel_types);
		}
		self
	}

	pub fn build(self) -> Component {
		if let Component::SelectMenu(select_menu) = &self.select_menu {
			if select_menu.options.is_none() {
				panic!("SelectMenu must have at least one option");
			}
			if select_menu.kind != SelectMenuType::Channel && select_menu.channel_types.is_some() {
				panic!("channel_types is only valid for channel select menus");
			}
			if let (Some(max_values), Some(min_values)) =
				(select_menu.max_values, select_menu.min_values)
			{
				if max_values < min_values {
					panic!("max_values must be greater than or equal to min_values");
				}
			}
		}
		self.select_menu
	}
}

pub struct SelectMenuOptionBuilder {
	option: SelectMenuOption,
}

impl SelectMenuOptionBuilder {
	pub fn new(label: &str, value: &str) -> Self {
		Self {
			option: SelectMenuOption {
				default: false,
				description: None,
				emoji: None,
				label: label.to_owned(),
				value: value.to_owned(),
			},
		}
	}

	pub fn default(mut self, default: bool) -> Self {
		self.option.default = default;
		self
	}

	pub fn description(mut self, description: &str) -> Self {
		self.option.description = Some(description.to_owned());
		self
	}

	pub fn emoji(mut self, emoji: ReactionType) -> Self {
		self.option.emoji = Some(emoji);
		self
	}

	pub fn build(self) -> SelectMenuOption {
		self.option
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_button() {
		let button = ButtonBuilder::new(ButtonStyle::Primary)
			.custom_id("button")
			.label("Button")
			.disabled(false)
			.emoji(ReactionType::Unicode {
				name: "👍".to_owned(),
			})
			.build();

		if let Component::Button(button) = button {
			assert_eq!(button.custom_id.unwrap(), "button");
			assert_eq!(button.label.unwrap(), "Button");
			assert_eq!(button.disabled, false);
			assert_eq!(button.emoji.unwrap(), ReactionType::Unicode {
				name: "👍".to_owned()
			});
		} else {
			panic!("Expected Button component");
		}
	}

	#[test]
	#[should_panic]
	fn link_button_without_url() {
		ButtonBuilder::new(ButtonStyle::Link).build();
	}

	#[test]
	#[should_panic]
	fn non_link_button_with_url() {
		ButtonBuilder::new(ButtonStyle::Primary)
			.url("https://example.com")
			.build();
	}

	#[test]
	fn valid_action_row() {
		let button = ButtonBuilder::new(ButtonStyle::Primary).build();
		let action_row = ActionRowBuilder::new().add_component(button).build();

		if let Component::ActionRow(action_row) = action_row {
			assert_eq!(action_row.components.len(), 1);
		} else {
			panic!("Expected ActionRow component");
		}
	}

	#[test]
	#[should_panic]
	fn action_row_without_components() {
		ActionRowBuilder::new().build();
	}

	#[test]
	fn valid_select_menu() {
		let option = SelectMenuOptionBuilder::new("Option", "option").build();
		let select_menu = SelectMenuBuilder::new("select", SelectMenuType::Text)
			.set_options(vec![option])
			.placeholder("Placeholder")
			.min_values(1)
			.max_values(2)
			.build();

		if let Component::SelectMenu(select_menu) = select_menu {
			assert_eq!(select_menu.custom_id, "select");
			assert_eq!(select_menu.options.unwrap().len(), 1);
			assert_eq!(select_menu.placeholder.unwrap(), "Placeholder");
			assert_eq!(select_menu.min_values.unwrap(), 1);
			assert_eq!(select_menu.max_values.unwrap(), 2);
		} else {
			panic!("Expected SelectMenu component");
		}
	}

	#[test]
	#[should_panic]
	fn select_menu_without_options() {
		SelectMenuBuilder::new("select", SelectMenuType::Text).build();
	}

	#[test]
	#[should_panic]
	fn channel_select_menu_without_channel_types() {
		SelectMenuBuilder::new("select", SelectMenuType::Channel).build();
	}

	#[test]
	#[should_panic]
	fn non_channel_select_menu_with_channel_types() {
		SelectMenuBuilder::new("select", SelectMenuType::Text)
			.channel_types(vec![ChannelType::GuildText])
			.build();
	}

	#[test]
	#[should_panic]
	fn select_menu_max_values_less_than_min_values() {
		SelectMenuBuilder::new("select", SelectMenuType::Text)
			.min_values(2)
			.max_values(1)
			.build();
	}

	#[test]
	fn valid_select_menu_option() {
		let option = SelectMenuOptionBuilder::new("Option", "option")
			.default(true)
			.description("Description")
			.emoji(ReactionType::Unicode {
				name: "👍".to_owned(),
			})
			.build();

		assert_eq!(option.label, "Option");
		assert_eq!(option.value, "option");
		assert_eq!(option.default, true);
		assert_eq!(option.description.unwrap(), "Description");
		assert_eq!(option.emoji.unwrap(), ReactionType::Unicode {
			name: "👍".to_owned()
		});
	}
}
