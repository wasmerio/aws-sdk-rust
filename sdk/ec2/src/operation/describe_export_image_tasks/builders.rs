// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_export_image_tasks::_describe_export_image_tasks_output::DescribeExportImageTasksOutputBuilder;

pub use crate::operation::describe_export_image_tasks::_describe_export_image_tasks_input::DescribeExportImageTasksInputBuilder;

impl DescribeExportImageTasksInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_export_image_tasks::DescribeExportImageTasksOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_export_image_tasks::DescribeExportImageTasksError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_export_image_tasks();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeExportImageTasks`.
///
/// <p>Describes the specified export image tasks or all of your export image tasks.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeExportImageTasksFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeExportImageTasksFluentBuilder {
    /// Creates a new `DescribeExportImageTasks`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeExportImageTasks as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksInputBuilder {
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
        crate::operation::describe_export_image_tasks::DescribeExportImageTasksOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_export_image_tasks::DescribeExportImageTasksError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_export_image_tasks::DescribeExportImageTasks::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_export_image_tasks::DescribeExportImageTasks::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_export_image_tasks::DescribeExportImageTasksOutput,
            crate::operation::describe_export_image_tasks::DescribeExportImageTasksError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_export_image_tasks::DescribeExportImageTasksError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_export_image_tasks::paginator::DescribeExportImageTasksPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_export_image_tasks::paginator::DescribeExportImageTasksPaginator {
        crate::operation::describe_export_image_tasks::paginator::DescribeExportImageTasksPaginator::new(self.handle, self.inner)
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filter tasks using the <code>task-state</code> filter and one of the following values: <code>active</code>, <code>completed</code>, <code>deleting</code>, or <code>deleted</code>.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Filter tasks using the <code>task-state</code> filter and one of the following values: <code>active</code>, <code>completed</code>, <code>deleting</code>, or <code>deleted</code>.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Filter tasks using the <code>task-state</code> filter and one of the following values: <code>active</code>, <code>completed</code>, <code>deleting</code>, or <code>deleted</code>.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// Appends an item to `ExportImageTaskIds`.
    ///
    /// To override the contents of this collection use [`set_export_image_task_ids`](Self::set_export_image_task_ids).
    ///
    /// <p>The IDs of the export image tasks.</p>
    pub fn export_image_task_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.export_image_task_ids(input.into());
        self
    }
    /// <p>The IDs of the export image tasks.</p>
    pub fn set_export_image_task_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_export_image_task_ids(input);
        self
    }
    /// <p>The IDs of the export image tasks.</p>
    pub fn get_export_image_task_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_export_image_task_ids()
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
