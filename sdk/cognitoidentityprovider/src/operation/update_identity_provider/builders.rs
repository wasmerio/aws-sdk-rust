// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_identity_provider::_update_identity_provider_output::UpdateIdentityProviderOutputBuilder;

pub use crate::operation::update_identity_provider::_update_identity_provider_input::UpdateIdentityProviderInputBuilder;

impl UpdateIdentityProviderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_identity_provider::UpdateIdentityProviderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_identity_provider::UpdateIdentityProviderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_identity_provider();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateIdentityProvider`.
///
/// <p>Updates IdP information for a user pool.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIdentityProviderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_identity_provider::builders::UpdateIdentityProviderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateIdentityProviderFluentBuilder {
    /// Creates a new `UpdateIdentityProvider`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateIdentityProvider as a reference.
    pub fn as_input(&self) -> &crate::operation::update_identity_provider::builders::UpdateIdentityProviderInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_identity_provider::UpdateIdentityProviderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_identity_provider::UpdateIdentityProviderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_identity_provider::UpdateIdentityProvider::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_identity_provider::UpdateIdentityProvider::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_identity_provider::UpdateIdentityProviderOutput,
            crate::operation::update_identity_provider::UpdateIdentityProviderError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_identity_provider::UpdateIdentityProviderError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user pool ID.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The IdP name.</p>
    pub fn provider_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provider_name(input.into());
        self
    }
    /// <p>The IdP name.</p>
    pub fn set_provider_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provider_name(input);
        self
    }
    /// <p>The IdP name.</p>
    pub fn get_provider_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_provider_name()
    }
    /// Adds a key-value pair to `ProviderDetails`.
    ///
    /// To override the contents of this collection use [`set_provider_details`](Self::set_provider_details).
    ///
    /// <p>The IdP details to be updated, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>
    pub fn provider_details(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.provider_details(k.into(), v.into());
        self
    }
    /// <p>The IdP details to be updated, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>
    pub fn set_provider_details(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_provider_details(input);
        self
    }
    /// <p>The IdP details to be updated, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>
    pub fn get_provider_details(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_provider_details()
    }
    /// Adds a key-value pair to `AttributeMapping`.
    ///
    /// To override the contents of this collection use [`set_attribute_mapping`](Self::set_attribute_mapping).
    ///
    /// <p>The IdP attribute mapping to be changed.</p>
    pub fn attribute_mapping(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.attribute_mapping(k.into(), v.into());
        self
    }
    /// <p>The IdP attribute mapping to be changed.</p>
    pub fn set_attribute_mapping(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_attribute_mapping(input);
        self
    }
    /// <p>The IdP attribute mapping to be changed.</p>
    pub fn get_attribute_mapping(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_attribute_mapping()
    }
    /// Appends an item to `IdpIdentifiers`.
    ///
    /// To override the contents of this collection use [`set_idp_identifiers`](Self::set_idp_identifiers).
    ///
    /// <p>A list of IdP identifiers.</p>
    pub fn idp_identifiers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.idp_identifiers(input.into());
        self
    }
    /// <p>A list of IdP identifiers.</p>
    pub fn set_idp_identifiers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_idp_identifiers(input);
        self
    }
    /// <p>A list of IdP identifiers.</p>
    pub fn get_idp_identifiers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_idp_identifiers()
    }
}
