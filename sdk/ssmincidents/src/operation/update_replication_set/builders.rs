// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_replication_set::_update_replication_set_output::UpdateReplicationSetOutputBuilder;

pub use crate::operation::update_replication_set::_update_replication_set_input::UpdateReplicationSetInputBuilder;

impl UpdateReplicationSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_replication_set::UpdateReplicationSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_replication_set::UpdateReplicationSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_replication_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateReplicationSet`.
///
/// <p>Add or delete Regions from your replication set.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateReplicationSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_replication_set::builders::UpdateReplicationSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateReplicationSetFluentBuilder {
    /// Creates a new `UpdateReplicationSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateReplicationSet as a reference.
    pub fn as_input(&self) -> &crate::operation::update_replication_set::builders::UpdateReplicationSetInputBuilder {
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
        crate::operation::update_replication_set::UpdateReplicationSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_replication_set::UpdateReplicationSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_replication_set::UpdateReplicationSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_replication_set::UpdateReplicationSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_replication_set::UpdateReplicationSetOutput,
            crate::operation::update_replication_set::UpdateReplicationSetError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_replication_set::UpdateReplicationSetError>,
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
    /// <p>The Amazon Resource Name (ARN) of the replication set you're updating.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the replication set you're updating.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the replication set you're updating.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// Appends an item to `actions`.
    ///
    /// To override the contents of this collection use [`set_actions`](Self::set_actions).
    ///
    /// <p>An action to add or delete a Region.</p>
    pub fn actions(mut self, input: crate::types::UpdateReplicationSetAction) -> Self {
        self.inner = self.inner.actions(input);
        self
    }
    /// <p>An action to add or delete a Region.</p>
    pub fn set_actions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UpdateReplicationSetAction>>) -> Self {
        self.inner = self.inner.set_actions(input);
        self
    }
    /// <p>An action to add or delete a Region.</p>
    pub fn get_actions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UpdateReplicationSetAction>> {
        self.inner.get_actions()
    }
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
