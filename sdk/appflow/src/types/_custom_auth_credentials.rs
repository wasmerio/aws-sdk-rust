// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The custom credentials required for custom authentication.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomAuthCredentials {
    /// <p>The custom authentication type that the connector uses.</p>
    pub custom_authentication_type: ::std::option::Option<::std::string::String>,
    /// <p>A map that holds custom authentication credentials.</p>
    pub credentials_map: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CustomAuthCredentials {
    /// <p>The custom authentication type that the connector uses.</p>
    pub fn custom_authentication_type(&self) -> ::std::option::Option<&str> {
        self.custom_authentication_type.as_deref()
    }
    /// <p>A map that holds custom authentication credentials.</p>
    pub fn credentials_map(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.credentials_map.as_ref()
    }
}
impl CustomAuthCredentials {
    /// Creates a new builder-style object to manufacture [`CustomAuthCredentials`](crate::types::CustomAuthCredentials).
    pub fn builder() -> crate::types::builders::CustomAuthCredentialsBuilder {
        crate::types::builders::CustomAuthCredentialsBuilder::default()
    }
}

/// A builder for [`CustomAuthCredentials`](crate::types::CustomAuthCredentials).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CustomAuthCredentialsBuilder {
    pub(crate) custom_authentication_type: ::std::option::Option<::std::string::String>,
    pub(crate) credentials_map: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl CustomAuthCredentialsBuilder {
    /// <p>The custom authentication type that the connector uses.</p>
    pub fn custom_authentication_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.custom_authentication_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom authentication type that the connector uses.</p>
    pub fn set_custom_authentication_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.custom_authentication_type = input;
        self
    }
    /// <p>The custom authentication type that the connector uses.</p>
    pub fn get_custom_authentication_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.custom_authentication_type
    }
    /// Adds a key-value pair to `credentials_map`.
    ///
    /// To override the contents of this collection use [`set_credentials_map`](Self::set_credentials_map).
    ///
    /// <p>A map that holds custom authentication credentials.</p>
    pub fn credentials_map(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.credentials_map.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.credentials_map = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map that holds custom authentication credentials.</p>
    pub fn set_credentials_map(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.credentials_map = input;
        self
    }
    /// <p>A map that holds custom authentication credentials.</p>
    pub fn get_credentials_map(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.credentials_map
    }
    /// Consumes the builder and constructs a [`CustomAuthCredentials`](crate::types::CustomAuthCredentials).
    pub fn build(self) -> crate::types::CustomAuthCredentials {
        crate::types::CustomAuthCredentials {
            custom_authentication_type: self.custom_authentication_type,
            credentials_map: self.credentials_map,
        }
    }
}
