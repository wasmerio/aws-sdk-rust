// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_flow_media_stream::_remove_flow_media_stream_output::RemoveFlowMediaStreamOutputBuilder;

pub use crate::operation::remove_flow_media_stream::_remove_flow_media_stream_input::RemoveFlowMediaStreamInputBuilder;

impl RemoveFlowMediaStreamInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_flow_media_stream();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveFlowMediaStream`.
///
/// Removes a media stream from a flow. This action is only available if the media stream is not associated with a source or output.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveFlowMediaStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_flow_media_stream::builders::RemoveFlowMediaStreamInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl RemoveFlowMediaStreamFluentBuilder {
    /// Creates a new `RemoveFlowMediaStream`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveFlowMediaStream as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_flow_media_stream::builders::RemoveFlowMediaStreamInputBuilder {
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
        crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_flow_media_stream::RemoveFlowMediaStream::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_flow_media_stream::RemoveFlowMediaStream::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamOutput,
            crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::remove_flow_media_stream::RemoveFlowMediaStreamError>,
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
    /// The Amazon Resource Name (ARN) of the flow.
    pub fn flow_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_arn(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of the flow.
    pub fn set_flow_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_arn(input);
        self
    }
    /// The Amazon Resource Name (ARN) of the flow.
    pub fn get_flow_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_arn()
    }
    /// The name of the media stream that you want to remove.
    pub fn media_stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.media_stream_name(input.into());
        self
    }
    /// The name of the media stream that you want to remove.
    pub fn set_media_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_media_stream_name(input);
        self
    }
    /// The name of the media stream that you want to remove.
    pub fn get_media_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_media_stream_name()
    }
}
