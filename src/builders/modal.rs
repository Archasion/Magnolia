#![allow(dead_code)]

use std::error::Error;

use twilight_model::channel::message::component::{TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{
	InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_validate::component::{
	action_row as validate_action_row, text_input as validate_text_input, ComponentValidationError,
};

const MODAL_COMPONENT_COUNT: usize = 5;
const MODAL_TITLE_LENGTH: usize = 45;
const MODAL_CUSTOM_ID_LENGTH: usize = 100;

/// Builder to create a [`Component::TextInput`].
#[derive(Debug, Clone)]
#[must_use = "must be used in an action row builder"]
pub struct TextInputBuilder(TextInput);

impl TextInputBuilder {
	/// Create a new [`Component::TextInput`] builder.
	pub fn new(
		label: impl Into<String>,
		custom_id: impl Into<String>,
		style: TextInputStyle,
	) -> Self {
		Self(TextInput {
			custom_id: custom_id.into(),
			label: label.into(),
			max_length: None,
			min_length: None,
			placeholder: None,
			required: None,
			value: None,
			style,
		})
	}

	/// Set the maximum length of the text input.
	///
	/// Defaults to not being specified, which uses Discord's default.
	pub const fn max_length(mut self, max_length: u16) -> Self {
		self.0.max_length = Some(max_length);
		self
	}

	/// Set the minimum length of the text input.
	///
	/// Defaults to not being specified, which uses Discord's default.
	pub const fn min_length(mut self, min_length: u16) -> Self {
		self.0.min_length = Some(min_length);
		self
	}

	/// Set the placeholder text for the text input.
	///
	/// Defaults to [`None`].
	pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
		self.0.placeholder = Some(placeholder.into());
		self
	}

	/// Set whether the text input is required.
	///
	/// Defaults to not being specified, which uses Discord's default.
	pub const fn required(mut self, required: bool) -> Self {
		self.0.required = Some(required);
		self
	}

	/// Set the initial value of the text input.
	///
	/// Defaults to [`None`].
	pub fn value(mut self, value: impl Into<String>) -> Self {
		self.0.value = Some(value.into());
		self
	}

	/// Ensure the text input is valid.
	///
	/// # Errors
	///
	/// Refer to the errors section of [`twilight_validate::component::text_input`]
	/// for possible errors.
	pub fn validate(self) -> Result<Self, ComponentValidationError> {
		validate_text_input(&self.0)?;
		Ok(self)
	}

	/// Consume the builder, returning a [`Component::TextInput`].
	#[must_use = "must be used in an action row builder"]
	pub fn build(self) -> Component {
		Component::TextInput(self.0)
	}
}

/// Builder to create a modal interaction response.
#[derive(Debug, Clone)]
#[must_use = "must be used to build a modal"]
pub struct ModalBuilder(InteractionResponseData);

impl ModalBuilder {
	/// Create a new modal builder.
	pub fn new(title: impl Into<String>, custom_id: impl Into<String>) -> Self {
		Self(InteractionResponseData {
			allowed_mentions: None,
			attachments: None,
			choices: None,
			components: None,
			content: None,
			custom_id: Some(custom_id.into()),
			embeds: None,
			flags: None,
			title: Some(title.into()),
			tts: None,
		})
	}

	/// Set the components of the modal.
	///
	/// Defaults to an empty vector.
	pub fn set_components(mut self, components: impl Into<Vec<Component>>) -> Self {
		self.0.components = Some(components.into());
		self
	}

	/// Add a component to the modal.
	///
	/// Defaults to an empty vector.
	pub fn add_component(mut self, component: Component) -> Self {
		self.0
			.components
			.get_or_insert_with(Vec::new)
			.push(component);

		self
	}

	/// Ensure the modal is valid.
	pub fn validate(self) -> Result<Self, Box<dyn Error + Send + Sync>> {
		if let Some(title) = &self.0.title {
			// Ensure title is not empty
			if title.is_empty() {
				return Err("Title must not be empty".into());
			}

			// Ensure title does not exceed the maximum length
			if title.len() > MODAL_TITLE_LENGTH {
				return Err(
					format!("Title must not exceed {MODAL_TITLE_LENGTH} characters").into(),
				);
			}
		} else {
			return Err("Title must not be empty".into());
		}

		if let Some(custom_id) = &self.0.custom_id {
			// Ensure custom ID is not empty
			if custom_id.is_empty() {
				return Err("Custom ID must not be empty".into());
			}

			// Ensure custom ID does not exceed the maximum length
			if custom_id.len() > MODAL_CUSTOM_ID_LENGTH {
				return Err(format!(
					"Custom ID must not exceed {MODAL_CUSTOM_ID_LENGTH} characters"
				)
				.into());
			}
		} else {
			return Err("Custom ID must not be empty".into());
		}

		// Ensure components are not empty
		if self.0.components.is_none() {
			return Err("Modal must have at least one component".into());
		}

		// Ensure the number of components does not exceed the maximum
		if self.0.components.as_ref().unwrap().len() > MODAL_COMPONENT_COUNT {
			return Err(format!(
				"Modal must not have more than {MODAL_COMPONENT_COUNT} components"
			)
			.into());
		}

		for component in self.0.components.as_ref().unwrap() {
			if let Component::ActionRow(action_row) = component {
				// Ensure ActionRow contains exactly one TextInput component
				if action_row.components.len() != 1 {
					return Err("ActionRow must contain exactly one TextInput component".into());
				}

				validate_action_row(action_row)?;
			} else {
				return Err("Modal must only contain ActionRow components".into());
			}
		}

		Ok(self)
	}

	/// Consume the builder, returning an [`InteractionResponse`].
	#[must_use = "must be used to build a modal"]
	pub fn build(self) -> InteractionResponse {
		InteractionResponse {
			kind: InteractionResponseType::Modal,
			data: Some(self.0),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::model::test::{modal_action_row, CUSTOM_ID, TEXT};

	#[test]
	fn text_input() {
		let text_input = TextInputBuilder::new(TEXT, CUSTOM_ID, TextInputStyle::Short)
			.max_length(10)
			.min_length(5)
			.placeholder(TEXT)
			.required(true)
			.value(TEXT)
			.validate()
			.expect("expected valid text input")
			.build();

		let Component::TextInput(text_input) = text_input else {
			panic!("expected text input component");
		};

		assert_eq!(text_input.custom_id, CUSTOM_ID);
		assert_eq!(text_input.label, TEXT);
		assert_eq!(text_input.max_length, Some(10));
		assert_eq!(text_input.min_length, Some(5));
		assert_eq!(text_input.placeholder, Some(TEXT.to_owned()));
		assert_eq!(text_input.required, Some(true));
		assert_eq!(text_input.value, Some(TEXT.to_owned()));
	}

	#[test]
	fn modal() {
		let action_row = modal_action_row();
		let modal = ModalBuilder::new(TEXT, CUSTOM_ID)
			.set_components([action_row.clone()])
			.add_component(action_row)
			.validate()
			.expect("expected valid modal")
			.build();

		assert_eq!(modal.kind, InteractionResponseType::Modal);

		let Some(data) = modal.data else {
			panic!("expected data");
		};

		assert_eq!(data.title, Some(TEXT.to_owned()));
		assert_eq!(data.custom_id, Some(CUSTOM_ID.to_owned()));

		let Some(components) = data.components else {
			panic!("expected components");
		};

		assert_eq!(components.len(), 2);
	}
}
