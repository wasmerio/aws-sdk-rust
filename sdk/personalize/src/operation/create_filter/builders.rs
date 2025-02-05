// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_filter::_create_filter_output::CreateFilterOutputBuilder;

pub use crate::operation::create_filter::_create_filter_input::CreateFilterInputBuilder;

impl CreateFilterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_filter::CreateFilterOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_filter::CreateFilterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_filter();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateFilter`.
///
/// <p>Creates a recommendation filter. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFilterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_filter::builders::CreateFilterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateFilterFluentBuilder {
    /// Creates a new `CreateFilter`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateFilter as a reference.
    pub fn as_input(&self) -> &crate::operation::create_filter::builders::CreateFilterInputBuilder {
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
        crate::operation::create_filter::CreateFilterOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_filter::CreateFilterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_filter::CreateFilter::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_filter::CreateFilter::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_filter::CreateFilterOutput,
            crate::operation::create_filter::CreateFilterError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_filter::CreateFilterError>,
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
    /// <p>The name of the filter to create.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the filter to create.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the filter to create.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The ARN of the dataset group that the filter will belong to.</p>
    pub fn dataset_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_group_arn(input.into());
        self
    }
    /// <p>The ARN of the dataset group that the filter will belong to.</p>
    pub fn set_dataset_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_group_arn(input);
        self
    }
    /// <p>The ARN of the dataset group that the filter will belong to.</p>
    pub fn get_dataset_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dataset_group_arn()
    }
    /// <p>The filter expression defines which items are included or excluded from recommendations. Filter expression must follow specific format rules. For information about filter expression structure and syntax, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-expressions.html">Filter expressions</a>.</p>
    pub fn filter_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter_expression(input.into());
        self
    }
    /// <p>The filter expression defines which items are included or excluded from recommendations. Filter expression must follow specific format rules. For information about filter expression structure and syntax, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-expressions.html">Filter expressions</a>.</p>
    pub fn set_filter_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_filter_expression(input);
        self
    }
    /// <p>The filter expression defines which items are included or excluded from recommendations. Filter expression must follow specific format rules. For information about filter expression structure and syntax, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-expressions.html">Filter expressions</a>.</p>
    pub fn get_filter_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_filter_expression()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the filter.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the filter.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the filter.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
