// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The color configurations for a column.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ColorsConfiguration {
    /// <p>A list of up to 50 custom colors.</p>
    pub custom_colors: ::std::option::Option<::std::vec::Vec<crate::types::CustomColor>>,
}
impl ColorsConfiguration {
    /// <p>A list of up to 50 custom colors.</p>
    pub fn custom_colors(&self) -> ::std::option::Option<&[crate::types::CustomColor]> {
        self.custom_colors.as_deref()
    }
}
impl ColorsConfiguration {
    /// Creates a new builder-style object to manufacture [`ColorsConfiguration`](crate::types::ColorsConfiguration).
    pub fn builder() -> crate::types::builders::ColorsConfigurationBuilder {
        crate::types::builders::ColorsConfigurationBuilder::default()
    }
}

/// A builder for [`ColorsConfiguration`](crate::types::ColorsConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ColorsConfigurationBuilder {
    pub(crate) custom_colors: ::std::option::Option<::std::vec::Vec<crate::types::CustomColor>>,
}
impl ColorsConfigurationBuilder {
    /// Appends an item to `custom_colors`.
    ///
    /// To override the contents of this collection use [`set_custom_colors`](Self::set_custom_colors).
    ///
    /// <p>A list of up to 50 custom colors.</p>
    pub fn custom_colors(mut self, input: crate::types::CustomColor) -> Self {
        let mut v = self.custom_colors.unwrap_or_default();
        v.push(input);
        self.custom_colors = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of up to 50 custom colors.</p>
    pub fn set_custom_colors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CustomColor>>) -> Self {
        self.custom_colors = input;
        self
    }
    /// <p>A list of up to 50 custom colors.</p>
    pub fn get_custom_colors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CustomColor>> {
        &self.custom_colors
    }
    /// Consumes the builder and constructs a [`ColorsConfiguration`](crate::types::ColorsConfiguration).
    pub fn build(self) -> crate::types::ColorsConfiguration {
        crate::types::ColorsConfiguration {
            custom_colors: self.custom_colors,
        }
    }
}
