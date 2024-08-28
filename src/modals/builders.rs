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
		if let Component::TextInput(text_input) = &self.text_input {
			if let (Some(max_length), Some(min_length)) =
				(text_input.max_length, text_input.min_length)
			{
				if max_length < min_length {
					panic!("max_length must be greater than or equal to min_length");
				}
			}
		}
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
	pub fn new(title: &str, custom_id: &str) -> Self {
		Self {
			components: Vec::new(),
			modal: InteractionResponseDataBuilder::new()
				.title(title)
				.custom_id(custom_id),
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

	pub fn build(self) -> InteractionResponse {
		if self.components.is_empty() {
			panic!("Modal must have at least one component");
		}
		InteractionResponse {
			kind: InteractionResponseType::Modal,
			data: Some(self.modal.components(self.components).build()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_text_input_builder() {
		let text_input = TextInputBuilder::new("Label", "custom_id", TextInputStyle::Short)
			.max_length(10)
			.min_length(5)
			.placeholder("Placeholder")
			.required(true)
			.value("Value")
			.build();

		if let Component::ActionRow(action_row) = text_input {
			if let Component::TextInput(text_input) = &action_row.components[0] {
				assert_eq!(text_input.custom_id, "custom_id");
				assert_eq!(text_input.label, "Label");
				assert_eq!(text_input.max_length, Some(10));
				assert_eq!(text_input.min_length, Some(5));
				assert_eq!(text_input.placeholder, Some("Placeholder".to_owned()));
				assert_eq!(text_input.required, Some(true));
				assert_eq!(text_input.value, Some("Value".to_owned()));
			} else {
				panic!("Expected TextInput component");
			}
		} else {
			panic!("Expected ActionRow component");
		}
	}

	#[test]
	#[should_panic]
	fn text_input_max_length_less_than_min_length() {
		TextInputBuilder::new("Label", "custom_id", TextInputStyle::Short)
			.max_length(5)
			.min_length(10)
			.build();
	}

	#[test]
	fn valid_modal_builder() {
		let text_input = TextInputBuilder::new("Label", "custom_id", TextInputStyle::Short).build();
		let modal = ModalBuilder::new("Title", "custom_id")
			.add_component(text_input)
			.build();

		assert_eq!(modal.kind, InteractionResponseType::Modal);
		assert_eq!(modal.data.is_some(), true);
	}

	#[test]
	#[should_panic]
	fn modal_without_components() {
		ModalBuilder::new("Title", "custom_id").build();
	}
}
