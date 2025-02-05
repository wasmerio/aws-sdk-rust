// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deliver_config_snapshot::_deliver_config_snapshot_output::DeliverConfigSnapshotOutputBuilder;

pub use crate::operation::deliver_config_snapshot::_deliver_config_snapshot_input::DeliverConfigSnapshotInputBuilder;

impl DeliverConfigSnapshotInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deliver_config_snapshot::DeliverConfigSnapshotOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deliver_config_snapshot();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeliverConfigSnapshot`.
///
/// <p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, Config sends the following notifications using an Amazon SNS topic that you have specified.</p>
/// <ul>
/// <li> <p>Notification of the start of the delivery.</p> </li>
/// <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li>
/// <li> <p>Notification of delivery failure, if the delivery failed.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeliverConfigSnapshotFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeliverConfigSnapshotFluentBuilder {
    /// Creates a new `DeliverConfigSnapshot`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeliverConfigSnapshot as a reference.
    pub fn as_input(&self) -> &crate::operation::deliver_config_snapshot::builders::DeliverConfigSnapshotInputBuilder {
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
        crate::operation::deliver_config_snapshot::DeliverConfigSnapshotOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deliver_config_snapshot::DeliverConfigSnapshot::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deliver_config_snapshot::DeliverConfigSnapshot::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshotOutput,
            crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::deliver_config_snapshot::DeliverConfigSnapshotError>,
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
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    pub fn delivery_channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.delivery_channel_name(input.into());
        self
    }
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    pub fn set_delivery_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_delivery_channel_name(input);
        self
    }
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    pub fn get_delivery_channel_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_delivery_channel_name()
    }
}
