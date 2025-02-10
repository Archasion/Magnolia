#![allow(dead_code)]

use twilight_model::channel::message::component::{
    ActionRow, Button, ButtonStyle, SelectMenu, SelectMenuOption, SelectMenuType,
};
use twilight_model::channel::message::{Component, EmojiReactionType};
use twilight_model::channel::ChannelType;
use twilight_model::id::marker::SkuMarker;
use twilight_model::id::Id;
use twilight_validate::component::{
    action_row as validate_action_row, button as validate_button,
    select_menu as validate_select_menu, ComponentValidationError,
};

/// Builder to create a [`Component::Button`].
#[derive(Debug, Clone)]
#[must_use = "must be used in an action row builder"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// Create a new [`Component::Button`] builder.
    pub fn new(custom_id: impl Into<String>, style: ButtonStyle) -> Self {
        Self(Button {
            custom_id: Some(custom_id.into()),
            disabled: false,
            emoji: None,
            label: None,
            url: None,
            sku_id: None,
            style,
        })
    }

    /// Set the label for the button.
    ///
    /// Defaults to [`None`].
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.0.label = Some(label.into());
        self
    }

    /// Set whether the button can be interacted with.
    ///
    /// Defaults to `false`.
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;
        self
    }

    /// Set an emoji icon for the button.
    ///
    /// Defaults to [`None`].
    pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
        self.0.emoji = Some(emoji);
        self
    }

    /// Set the URL for a link button.
    ///
    /// Defaults to [`None`].
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url = Some(url.into());
        self
    }

    /// Set the SKU ID for a buy button.
    ///
    /// Defaults to [`None`].
    pub fn sku_id(mut self, sku_id: impl Into<Id<SkuMarker>>) -> Self {
        self.0.sku_id = Some(sku_id.into());
        self
    }

    /// Ensure the button is valid.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of [`twilight_validate::component::button`]
    /// for possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        validate_button(&self.0)?;
        Ok(self)
    }

    /// Consume the builder, returning a [`Component::Button`].
    #[must_use = "must be used in an action row builder"]
    pub fn build(self) -> Component {
        Component::Button(self.0)
    }
}

/// Builder to create a [`Component::ActionRow`].
#[derive(Debug, Clone)]
#[must_use = "must be built into an action row"]
pub struct ActionRowBuilder(ActionRow);

impl ActionRowBuilder {
    /// Create a new [`Component::ActionRow`] builder.
    pub const fn new() -> Self {
        Self(ActionRow {
            components: Vec::new(),
        })
    }

    /// Set the components for the action row.
    ///
    /// Defaults to an empty vector.
    pub fn set_components(mut self, components: impl Into<Vec<Component>>) -> Self {
        self.0.components = components.into();
        self
    }

    /// Add a component to the action row.
    ///
    /// Defaults to an empty vector.
    pub fn add_component(mut self, component: Component) -> Self {
        self.0.components.push(component);
        self
    }

    /// Ensure the action row is valid.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of [`twilight_validate::component::action_row`]
    /// for possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        validate_action_row(&self.0)?;
        Ok(self)
    }

    /// Consume the builder, returning a [`Component::ActionRow`].
    #[must_use = "must be built into an action row"]
    pub fn build(self) -> Component {
        Component::ActionRow(self.0)
    }
}

/// Builder to create a [`Component::SelectMenu`].
#[derive(Debug, Clone)]
#[must_use = "must be used in an action row builder"]
pub struct SelectMenuBuilder(SelectMenu);

impl SelectMenuBuilder {
    /// Create a new [`Component::SelectMenu`] builder.
    pub fn new(custom_id: impl Into<String>, kind: SelectMenuType) -> Self {
        Self(SelectMenu {
            channel_types: None,
            custom_id: custom_id.into(),
            default_values: None,
            disabled: false,
            options: None,
            placeholder: None,
            min_values: None,
            max_values: None,
            kind,
        })
    }

    /// Set whether the select menu can be interacted with.
    ///
    /// Defaults to `false`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;
        self
    }

    /// Set the options for the select menu.
    ///
    /// Defaults to [`None`].
    pub fn set_options(mut self, options: impl Into<Vec<SelectMenuOption>>) -> Self {
        self.0.options = Some(options.into());
        self
    }

    /// Add an option to the select menu.
    ///
    /// Defaults to [`None`].
    pub fn add_option(mut self, option: SelectMenuOption) -> Self {
        if let Some(options) = &mut self.0.options {
            options.push(option);
        } else {
            self.0.options = Some(vec![option]);
        }
        self
    }

    /// Set the placeholder text for the select menu.
    ///
    /// Defaults to [`None`].
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.0.placeholder = Some(placeholder.into());
        self
    }

    /// Set the minimum number of values that can be selected.
    ///
    /// Defaults to not being specified, which uses Discord's default.
    pub fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values = Some(min_values);
        self
    }

    /// Set the maximum number of values that can be selected.
    ///
    /// Defaults to not being specified, which uses Discord's default.
    pub fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values = Some(max_values);
        self
    }

    /// Set the channel types that can appear in the select menu.
    ///
    /// Defaults to not being specified, which allows all channel types.
    pub fn channel_types(mut self, channel_types: impl Into<Vec<ChannelType>>) -> Self {
        self.0.channel_types = Some(channel_types.into());
        self
    }

    /// Ensure the select menu is valid.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of [`twilight_validate::component::select_menu`]
    /// for possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        validate_select_menu(&self.0)?;
        Ok(self)
    }

    /// Consume the builder, returning a [`Component::SelectMenu`].
    #[must_use = "must be used in an action row builder"]
    pub fn build(self) -> Component {
        Component::SelectMenu(self.0)
    }
}

/// Builder to create a [`SelectMenuOption`].
#[derive(Debug, Clone)]
#[must_use = "must be used in a select menu builder"]
pub struct SelectMenuOptionBuilder(SelectMenuOption);

impl SelectMenuOptionBuilder {
    /// Create a new [`SelectMenuOption`] builder.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self(SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: label.into(),
            value: value.into(),
        })
    }

    /// Set whether the option is the default.
    ///
    /// Defaults to `false`.
    pub fn default(mut self, default: bool) -> Self {
        self.0.default = default;
        self
    }

    /// Set the description for the option.
    ///
    /// Defaults to [`None`].
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description = Some(description.into());
        self
    }

    /// Set an emoji icon for the option.
    ///
    /// Defaults to [`None`].
    pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
        self.0.emoji = Some(emoji);
        self
    }

    /// Consume the builder, returning a [`SelectMenuOption`].
    #[must_use = "must be used in a select menu builder"]
    pub fn build(self) -> SelectMenuOption {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::test::{
        button as button_model, select_menu_option as select_menu_option_model, CUSTOM_ID, TEXT,
    };

    #[test]
    fn button() {
        let button = ButtonBuilder::new(CUSTOM_ID, ButtonStyle::Primary)
            .label(TEXT)
            .emoji(EmojiReactionType::Unicode {
                name: "👍".to_owned(),
            })
            .validate()
            .expect("failed to validate button")
            .build();

        let Component::Button(button) = button else {
            panic!("expected button component");
        };

        assert_eq!(button.custom_id.unwrap(), CUSTOM_ID);
        assert_eq!(button.label.unwrap(), TEXT);
        assert_eq!(button.disabled, false);
        assert_eq!(button.emoji.unwrap(), EmojiReactionType::Unicode {
            name: "👍".to_owned()
        });
    }

    #[test]
    fn action_row() {
        let button = button_model();
        let action_row = ActionRowBuilder::new()
            .set_components([button.clone()])
            .add_component(button)
            .validate()
            .expect("failed to validate action row")
            .build();

        let Component::ActionRow(action_row) = action_row else {
            panic!("expected action row component");
        };

        assert_eq!(action_row.components.len(), 2);
    }

    #[test]
    fn select_menu_option() {
        let option = SelectMenuOptionBuilder::new(TEXT, CUSTOM_ID)
            .default(true)
            .description(TEXT)
            .emoji(EmojiReactionType::Unicode {
                name: "👍".to_owned(),
            })
            .build();

        assert_eq!(option.label, TEXT);
        assert_eq!(option.value, CUSTOM_ID);
        assert_eq!(option.default, true);
        assert_eq!(option.description.unwrap(), TEXT);
        assert_eq!(option.emoji.unwrap(), EmojiReactionType::Unicode {
            name: "👍".to_owned()
        });
    }

    #[test]
    fn select_menu() {
        let option = select_menu_option_model();
        let select_menu = SelectMenuBuilder::new(CUSTOM_ID, SelectMenuType::Text)
            .set_options([option.clone()])
            .add_option(option)
            .placeholder(TEXT)
            .min_values(1)
            .max_values(2)
            .validate()
            .expect("failed to validate select menu")
            .build();

        let Component::SelectMenu(select_menu) = select_menu else {
            panic!("expected select menu component");
        };

        assert_eq!(select_menu.custom_id, CUSTOM_ID);
        assert_eq!(select_menu.disabled, false);
        assert_eq!(select_menu.options.unwrap().len(), 2);
        assert_eq!(select_menu.placeholder.unwrap(), TEXT);
        assert_eq!(select_menu.min_values.unwrap(), 1);
        assert_eq!(select_menu.max_values.unwrap(), 2);
    }
}
