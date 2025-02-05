// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_participant::_get_participant_output::GetParticipantOutputBuilder;

pub use crate::operation::get_participant::_get_participant_input::GetParticipantInputBuilder;

impl GetParticipantInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_participant::GetParticipantOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_participant::GetParticipantError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_participant();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetParticipant`.
///
/// <p>Gets information about the specified participant token.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetParticipantFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_participant::builders::GetParticipantInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetParticipantFluentBuilder {
    /// Creates a new `GetParticipant`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetParticipant as a reference.
    pub fn as_input(&self) -> &crate::operation::get_participant::builders::GetParticipantInputBuilder {
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
        crate::operation::get_participant::GetParticipantOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_participant::GetParticipantError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_participant::GetParticipant::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_participant::GetParticipant::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_participant::GetParticipantOutput,
            crate::operation::get_participant::GetParticipantError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_participant::GetParticipantError>,
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
    /// <p>Stage ARN.</p>
    pub fn stage_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stage_arn(input.into());
        self
    }
    /// <p>Stage ARN.</p>
    pub fn set_stage_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stage_arn(input);
        self
    }
    /// <p>Stage ARN.</p>
    pub fn get_stage_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stage_arn()
    }
    /// <p>ID of a session within the stage.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>ID of a session within the stage.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>ID of a session within the stage.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
    /// <p>Unique identifier for the participant. This is assigned by IVS and returned by <code>CreateParticipantToken</code>.</p>
    pub fn participant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.participant_id(input.into());
        self
    }
    /// <p>Unique identifier for the participant. This is assigned by IVS and returned by <code>CreateParticipantToken</code>.</p>
    pub fn set_participant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_participant_id(input);
        self
    }
    /// <p>Unique identifier for the participant. This is assigned by IVS and returned by <code>CreateParticipantToken</code>.</p>
    pub fn get_participant_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_participant_id()
    }
}
