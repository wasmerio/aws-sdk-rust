// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_gateway_information::_update_gateway_information_output::UpdateGatewayInformationOutputBuilder;

pub use crate::operation::update_gateway_information::_update_gateway_information_input::UpdateGatewayInformationInputBuilder;

impl UpdateGatewayInformationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_gateway_information::UpdateGatewayInformationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_gateway_information::UpdateGatewayInformationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_gateway_information();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateGatewayInformation`.
///
/// <p>Updates a gateway's metadata, which includes the gateway's name and time zone. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p> <note>
/// <p>For gateways activated after September 2, 2015, the gateway's ARN contains the gateway ID rather than the gateway name. However, changing the name of the gateway has no effect on the gateway's ARN.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateGatewayInformationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_gateway_information::builders::UpdateGatewayInformationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateGatewayInformationFluentBuilder {
    /// Creates a new `UpdateGatewayInformation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateGatewayInformation as a reference.
    pub fn as_input(&self) -> &crate::operation::update_gateway_information::builders::UpdateGatewayInformationInputBuilder {
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
        crate::operation::update_gateway_information::UpdateGatewayInformationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_gateway_information::UpdateGatewayInformationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_gateway_information::UpdateGatewayInformation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_gateway_information::UpdateGatewayInformation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_gateway_information::UpdateGatewayInformationOutput,
            crate::operation::update_gateway_information::UpdateGatewayInformationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_gateway_information::UpdateGatewayInformationError>,
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
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn get_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_arn()
    }
    /// <p>The name you configured for your gateway.</p>
    pub fn gateway_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_name(input.into());
        self
    }
    /// <p>The name you configured for your gateway.</p>
    pub fn set_gateway_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_name(input);
        self
    }
    /// <p>The name you configured for your gateway.</p>
    pub fn get_gateway_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_name()
    }
    /// <p>A value that indicates the time zone of the gateway.</p>
    pub fn gateway_timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_timezone(input.into());
        self
    }
    /// <p>A value that indicates the time zone of the gateway.</p>
    pub fn set_gateway_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_timezone(input);
        self
    }
    /// <p>A value that indicates the time zone of the gateway.</p>
    pub fn get_gateway_timezone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_timezone()
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that you want to use to monitor and log events in the gateway.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html">What is Amazon CloudWatch Logs?</a> </p>
    pub fn cloud_watch_log_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cloud_watch_log_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that you want to use to monitor and log events in the gateway.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html">What is Amazon CloudWatch Logs?</a> </p>
    pub fn set_cloud_watch_log_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cloud_watch_log_group_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that you want to use to monitor and log events in the gateway.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html">What is Amazon CloudWatch Logs?</a> </p>
    pub fn get_cloud_watch_log_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cloud_watch_log_group_arn()
    }
    /// <p>Specifies the size of the gateway's metadata cache.</p>
    pub fn gateway_capacity(mut self, input: crate::types::GatewayCapacity) -> Self {
        self.inner = self.inner.gateway_capacity(input);
        self
    }
    /// <p>Specifies the size of the gateway's metadata cache.</p>
    pub fn set_gateway_capacity(mut self, input: ::std::option::Option<crate::types::GatewayCapacity>) -> Self {
        self.inner = self.inner.set_gateway_capacity(input);
        self
    }
    /// <p>Specifies the size of the gateway's metadata cache.</p>
    pub fn get_gateway_capacity(&self) -> &::std::option::Option<crate::types::GatewayCapacity> {
        self.inner.get_gateway_capacity()
    }
}
