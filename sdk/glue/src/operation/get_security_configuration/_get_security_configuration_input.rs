// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSecurityConfigurationInput {
    /// <p>The name of the security configuration to retrieve.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl GetSecurityConfigurationInput {
    /// <p>The name of the security configuration to retrieve.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl GetSecurityConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetSecurityConfigurationInput`](crate::operation::get_security_configuration::GetSecurityConfigurationInput).
    pub fn builder() -> crate::operation::get_security_configuration::builders::GetSecurityConfigurationInputBuilder {
        crate::operation::get_security_configuration::builders::GetSecurityConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetSecurityConfigurationInput`](crate::operation::get_security_configuration::GetSecurityConfigurationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetSecurityConfigurationInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl GetSecurityConfigurationInputBuilder {
    /// <p>The name of the security configuration to retrieve.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the security configuration to retrieve.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the security configuration to retrieve.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`GetSecurityConfigurationInput`](crate::operation::get_security_configuration::GetSecurityConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_security_configuration::GetSecurityConfigurationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_security_configuration::GetSecurityConfigurationInput { name: self.name })
    }
}
