// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_provisioning_template::_delete_provisioning_template_output::DeleteProvisioningTemplateOutputBuilder;

pub use crate::operation::delete_provisioning_template::_delete_provisioning_template_input::DeleteProvisioningTemplateInputBuilder;

impl DeleteProvisioningTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_provisioning_template::DeleteProvisioningTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_provisioning_template::DeleteProvisioningTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_provisioning_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteProvisioningTemplate`.
///
/// <p>Deletes a provisioning template.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">DeleteProvisioningTemplate</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteProvisioningTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_provisioning_template::builders::DeleteProvisioningTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteProvisioningTemplateFluentBuilder {
    /// Creates a new `DeleteProvisioningTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteProvisioningTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_provisioning_template::builders::DeleteProvisioningTemplateInputBuilder {
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
        crate::operation::delete_provisioning_template::DeleteProvisioningTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_provisioning_template::DeleteProvisioningTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_provisioning_template::DeleteProvisioningTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_provisioning_template::DeleteProvisioningTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_provisioning_template::DeleteProvisioningTemplateOutput,
            crate::operation::delete_provisioning_template::DeleteProvisioningTemplateError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_provisioning_template::DeleteProvisioningTemplateError>,
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
    /// <p>The name of the fleet provision template to delete.</p>
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the fleet provision template to delete.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the fleet provision template to delete.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_name()
    }
}
