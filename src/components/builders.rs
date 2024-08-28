#![allow(dead_code)]

use twilight_model::channel::message::component::{
	ActionRow, Button, ButtonStyle, SelectMenu, SelectMenuOption, SelectMenuType,
};
use twilight_model::channel::message::{Component, ReactionType};

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

	pub fn add_component(mut self, button: Button) -> Self {
		self.components.push(Component::Button(button));
		self
	}

	pub fn build(self) -> Component {
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

	pub fn build(self) -> Component {
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
