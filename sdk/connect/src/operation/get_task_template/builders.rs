// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_task_template::_get_task_template_output::GetTaskTemplateOutputBuilder;

pub use crate::operation::get_task_template::_get_task_template_input::GetTaskTemplateInputBuilder;

impl GetTaskTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_task_template::GetTaskTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_task_template::GetTaskTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_task_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTaskTemplate`.
///
/// <p>Gets details about a specific task template in the specified Amazon Connect instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTaskTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_task_template::builders::GetTaskTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetTaskTemplateFluentBuilder {
    /// Creates a new `GetTaskTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetTaskTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::get_task_template::builders::GetTaskTemplateInputBuilder {
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
        crate::operation::get_task_template::GetTaskTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_task_template::GetTaskTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_task_template::GetTaskTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_task_template::GetTaskTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_task_template::GetTaskTemplateOutput,
            crate::operation::get_task_template::GetTaskTemplateError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_task_template::GetTaskTemplateError>,
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
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>A unique identifier for the task template.</p>
    pub fn task_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task_template_id(input.into());
        self
    }
    /// <p>A unique identifier for the task template.</p>
    pub fn set_task_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_task_template_id(input);
        self
    }
    /// <p>A unique identifier for the task template.</p>
    pub fn get_task_template_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_task_template_id()
    }
    /// <p>The system generated version of a task template that is associated with a task, when the task is created.</p>
    pub fn snapshot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_version(input.into());
        self
    }
    /// <p>The system generated version of a task template that is associated with a task, when the task is created.</p>
    pub fn set_snapshot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_version(input);
        self
    }
    /// <p>The system generated version of a task template that is associated with a task, when the task is created.</p>
    pub fn get_snapshot_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_version()
    }
}
