// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable::_enable_output::EnableOutputBuilder;

pub use crate::operation::enable::_enable_input::EnableInputBuilder;

impl EnableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable::EnableOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::enable::EnableError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
    > {
        let mut fluent_builder = client.enable();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Enable`.
///
/// <p>Enables Amazon Inspector scans for one or more Amazon Web Services accounts.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable::builders::EnableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl EnableFluentBuilder {
    /// Creates a new `Enable`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the Enable as a reference.
    pub fn as_input(&self) -> &crate::operation::enable::builders::EnableInputBuilder {
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
        crate::operation::enable::EnableOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::enable::EnableError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::enable::Enable::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
        crate::operation::enable::Enable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<crate::operation::enable::EnableOutput, crate::operation::enable::EnableError>,
        ::aws_smithy_http::result::SdkError<crate::operation::enable::EnableError>,
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
    /// Appends an item to `accountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>A list of account IDs you want to enable Amazon Inspector scans for.</p>
    pub fn account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_ids(input.into());
        self
    }
    /// <p>A list of account IDs you want to enable Amazon Inspector scans for.</p>
    pub fn set_account_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
    /// <p>A list of account IDs you want to enable Amazon Inspector scans for.</p>
    pub fn get_account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_account_ids()
    }
    /// Appends an item to `resourceTypes`.
    ///
    /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
    ///
    /// <p>The resource scan types you want to enable.</p>
    pub fn resource_types(mut self, input: crate::types::ResourceScanType) -> Self {
        self.inner = self.inner.resource_types(input);
        self
    }
    /// <p>The resource scan types you want to enable.</p>
    pub fn set_resource_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceScanType>>) -> Self {
        self.inner = self.inner.set_resource_types(input);
        self
    }
    /// <p>The resource scan types you want to enable.</p>
    pub fn get_resource_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourceScanType>> {
        self.inner.get_resource_types()
    }
    /// <p>The idempotency token for the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token for the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The idempotency token for the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
