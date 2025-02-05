// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_multicast_group_from_fuota_task::_disassociate_multicast_group_from_fuota_task_output::DisassociateMulticastGroupFromFuotaTaskOutputBuilder;

pub use crate::operation::disassociate_multicast_group_from_fuota_task::_disassociate_multicast_group_from_fuota_task_input::DisassociateMulticastGroupFromFuotaTaskInputBuilder;

impl DisassociateMulticastGroupFromFuotaTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_multicast_group_from_fuota_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateMulticastGroupFromFuotaTask`.
///
/// <p>Disassociates a multicast group from a fuota task.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateMulticastGroupFromFuotaTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_multicast_group_from_fuota_task::builders::DisassociateMulticastGroupFromFuotaTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DisassociateMulticastGroupFromFuotaTaskFluentBuilder {
    /// Creates a new `DisassociateMulticastGroupFromFuotaTask`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateMulticastGroupFromFuotaTask as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::disassociate_multicast_group_from_fuota_task::builders::DisassociateMulticastGroupFromFuotaTaskInputBuilder {
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
        crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTask::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTask::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskOutput,
            crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskError,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_multicast_group_from_fuota_task::DisassociateMulticastGroupFromFuotaTaskError,
        >,
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
    /// <p>The ID of a FUOTA task.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of a FUOTA task.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of a FUOTA task.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The ID of the multicast group.</p>
    pub fn multicast_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.multicast_group_id(input.into());
        self
    }
    /// <p>The ID of the multicast group.</p>
    pub fn set_multicast_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_multicast_group_id(input);
        self
    }
    /// <p>The ID of the multicast group.</p>
    pub fn get_multicast_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_multicast_group_id()
    }
}
