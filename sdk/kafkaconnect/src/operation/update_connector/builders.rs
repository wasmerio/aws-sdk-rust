// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_connector::_update_connector_output::UpdateConnectorOutputBuilder;

pub use crate::operation::update_connector::_update_connector_input::UpdateConnectorInputBuilder;

impl UpdateConnectorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_connector::UpdateConnectorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_connector::UpdateConnectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_connector();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConnector`.
///
/// <p>Updates the specified connector.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConnectorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_connector::builders::UpdateConnectorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateConnectorFluentBuilder {
    /// Creates a new `UpdateConnector`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConnector as a reference.
    pub fn as_input(&self) -> &crate::operation::update_connector::builders::UpdateConnectorInputBuilder {
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
        crate::operation::update_connector::UpdateConnectorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_connector::UpdateConnectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_connector::UpdateConnector::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_connector::UpdateConnector::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_connector::UpdateConnectorOutput,
            crate::operation::update_connector::UpdateConnectorError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_connector::UpdateConnectorError>,
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
    /// <p>The target capacity.</p>
    pub fn capacity(mut self, input: crate::types::CapacityUpdate) -> Self {
        self.inner = self.inner.capacity(input);
        self
    }
    /// <p>The target capacity.</p>
    pub fn set_capacity(mut self, input: ::std::option::Option<crate::types::CapacityUpdate>) -> Self {
        self.inner = self.inner.set_capacity(input);
        self
    }
    /// <p>The target capacity.</p>
    pub fn get_capacity(&self) -> &::std::option::Option<crate::types::CapacityUpdate> {
        self.inner.get_capacity()
    }
    /// <p>The Amazon Resource Name (ARN) of the connector that you want to update.</p>
    pub fn connector_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the connector that you want to update.</p>
    pub fn set_connector_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the connector that you want to update.</p>
    pub fn get_connector_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_arn()
    }
    /// <p>The current version of the connector that you want to update.</p>
    pub fn current_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.current_version(input.into());
        self
    }
    /// <p>The current version of the connector that you want to update.</p>
    pub fn set_current_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_current_version(input);
        self
    }
    /// <p>The current version of the connector that you want to update.</p>
    pub fn get_current_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_current_version()
    }
}
