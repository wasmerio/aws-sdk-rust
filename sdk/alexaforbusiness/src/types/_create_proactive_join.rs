// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateProactiveJoin {
    #[allow(missing_docs)] // documentation missing in model
    pub enabled_by_motion: ::std::option::Option<bool>,
}
impl CreateProactiveJoin {
    #[allow(missing_docs)] // documentation missing in model
    pub fn enabled_by_motion(&self) -> ::std::option::Option<bool> {
        self.enabled_by_motion
    }
}
impl CreateProactiveJoin {
    /// Creates a new builder-style object to manufacture [`CreateProactiveJoin`](crate::types::CreateProactiveJoin).
    pub fn builder() -> crate::types::builders::CreateProactiveJoinBuilder {
        crate::types::builders::CreateProactiveJoinBuilder::default()
    }
}

/// A builder for [`CreateProactiveJoin`](crate::types::CreateProactiveJoin).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateProactiveJoinBuilder {
    pub(crate) enabled_by_motion: ::std::option::Option<bool>,
}
impl CreateProactiveJoinBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn enabled_by_motion(mut self, input: bool) -> Self {
        self.enabled_by_motion = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_enabled_by_motion(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled_by_motion = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_enabled_by_motion(&self) -> &::std::option::Option<bool> {
        &self.enabled_by_motion
    }
    /// Consumes the builder and constructs a [`CreateProactiveJoin`](crate::types::CreateProactiveJoin).
    pub fn build(self) -> crate::types::CreateProactiveJoin {
        crate::types::CreateProactiveJoin {
            enabled_by_motion: self.enabled_by_motion,
        }
    }
}
