// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_update_device_status::_admin_update_device_status_output::AdminUpdateDeviceStatusOutputBuilder;

pub use crate::operation::admin_update_device_status::_admin_update_device_status_input::AdminUpdateDeviceStatusInputBuilder;

impl AdminUpdateDeviceStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::admin_update_device_status::AdminUpdateDeviceStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_update_device_status::AdminUpdateDeviceStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.admin_update_device_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AdminUpdateDeviceStatus`.
///
/// <p>Updates the device status as an administrator.</p>
/// <p>Calling this action requires developer credentials.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AdminUpdateDeviceStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_update_device_status::builders::AdminUpdateDeviceStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AdminUpdateDeviceStatusFluentBuilder {
    /// Creates a new `AdminUpdateDeviceStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AdminUpdateDeviceStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::admin_update_device_status::builders::AdminUpdateDeviceStatusInputBuilder {
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
        crate::operation::admin_update_device_status::AdminUpdateDeviceStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_update_device_status::AdminUpdateDeviceStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::admin_update_device_status::AdminUpdateDeviceStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::admin_update_device_status::AdminUpdateDeviceStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::admin_update_device_status::AdminUpdateDeviceStatusOutput,
            crate::operation::admin_update_device_status::AdminUpdateDeviceStatusError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::admin_update_device_status::AdminUpdateDeviceStatusError>,
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
    /// <p>The user name.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The user name.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_username()
    }
    /// <p>The device key.</p>
    pub fn device_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_key(input.into());
        self
    }
    /// <p>The device key.</p>
    pub fn set_device_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_key(input);
        self
    }
    /// <p>The device key.</p>
    pub fn get_device_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_key()
    }
    /// <p>The status indicating whether a device has been remembered or not.</p>
    pub fn device_remembered_status(mut self, input: crate::types::DeviceRememberedStatusType) -> Self {
        self.inner = self.inner.device_remembered_status(input);
        self
    }
    /// <p>The status indicating whether a device has been remembered or not.</p>
    pub fn set_device_remembered_status(mut self, input: ::std::option::Option<crate::types::DeviceRememberedStatusType>) -> Self {
        self.inner = self.inner.set_device_remembered_status(input);
        self
    }
    /// <p>The status indicating whether a device has been remembered or not.</p>
    pub fn get_device_remembered_status(&self) -> &::std::option::Option<crate::types::DeviceRememberedStatusType> {
        self.inner.get_device_remembered_status()
    }
}
