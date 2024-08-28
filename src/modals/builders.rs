#![allow(dead_code)]

use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct TextInputBuilder {
	text_input: Component,
}

impl TextInputBuilder {
	pub fn new(label: &str, custom_id: &str, style: TextInputStyle) -> Self {
		Self {
			text_input: Component::TextInput(TextInput {
				custom_id: custom_id.to_owned(),
				label: label.to_owned(),
				max_length: None,
				min_length: None,
				placeholder: None,
				required: None,
				value: None,
				style,
			}),
		}
	}

	pub fn max_length(mut self, max_length: u16) -> Self {
		if let Component::TextInput(text_input) = &mut self.text_input {
			text_input.max_length = Some(max_length);
		}
		self
	}

	pub fn min_length(mut self, min_length: u16) -> Self {
		if let Component::TextInput(text_input) = &mut self.text_input {
			text_input.min_length = Some(min_length);
		}
		self
	}

	pub fn placeholder(mut self, placeholder: &str) -> Self {
		if let Component::TextInput(text_input) = &mut self.text_input {
			text_input.placeholder = Some(placeholder.to_owned());
		}
		self
	}

	pub fn required(mut self, required: bool) -> Self {
		if let Component::TextInput(text_input) = &mut self.text_input {
			text_input.required = Some(required);
		}
		self
	}

	pub fn value(mut self, value: &str) -> Self {
		if let Component::TextInput(text_input) = &mut self.text_input {
			text_input.value = Some(value.to_owned());
		}
		self
	}

	pub fn build(self) -> Component {
		Component::ActionRow(ActionRow {
			components: vec![self.text_input],
		})
	}
}

pub struct ModalBuilder {
	modal: InteractionResponseDataBuilder,
	components: Vec<Component>,
}

impl ModalBuilder {
	pub fn new() -> Self {
		Self {
			modal: InteractionResponseDataBuilder::new(),
			components: Vec::new(),
		}
	}

	pub fn custom_id(mut self, custom_id: &str) -> Self {
		self.modal = self.modal.custom_id(custom_id);
		self
	}

	pub fn title(mut self, title: &str) -> Self {
		self.modal = self.modal.title(title);
		self
	}

	pub fn set_components(mut self, components: Vec<Component>) -> Self {
		self.components = components;
		self
	}

	pub fn add_component(mut self, component: Component) -> Self {
		self.components.push(component);
		self
	}

	pub fn build(self) -> InteractionResponse {
		InteractionResponse {
			kind: InteractionResponseType::Modal,
			data: Some(self.modal.components(self.components).build()),
		}
	}
}
