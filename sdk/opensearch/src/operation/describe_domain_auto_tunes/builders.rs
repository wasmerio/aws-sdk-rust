// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_domain_auto_tunes::_describe_domain_auto_tunes_output::DescribeDomainAutoTunesOutputBuilder;

pub use crate::operation::describe_domain_auto_tunes::_describe_domain_auto_tunes_input::DescribeDomainAutoTunesInputBuilder;

impl DescribeDomainAutoTunesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_domain_auto_tunes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeDomainAutoTunes`.
///
/// <p>Returns the list of optimizations that Auto-Tune has made to an Amazon OpenSearch Service domain. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/auto-tune.html">Auto-Tune for Amazon OpenSearch Service</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeDomainAutoTunesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_domain_auto_tunes::builders::DescribeDomainAutoTunesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeDomainAutoTunesFluentBuilder {
    /// Creates a new `DescribeDomainAutoTunes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeDomainAutoTunes as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_domain_auto_tunes::builders::DescribeDomainAutoTunesInputBuilder {
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
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesOutput,
            crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_domain_auto_tunes::DescribeDomainAutoTunesError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_domain_auto_tunes::paginator::DescribeDomainAutoTunesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_domain_auto_tunes::paginator::DescribeDomainAutoTunesPaginator {
        crate::operation::describe_domain_auto_tunes::paginator::DescribeDomainAutoTunesPaginator::new(self.handle, self.inner)
    }
    /// <p>Name of the domain that you want Auto-Tune details about.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>Name of the domain that you want Auto-Tune details about.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>Name of the domain that you want Auto-Tune details about.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If your initial <code>DescribeDomainAutoTunes</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>DescribeDomainAutoTunes</code> operations, which returns results in the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If your initial <code>DescribeDomainAutoTunes</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>DescribeDomainAutoTunes</code> operations, which returns results in the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If your initial <code>DescribeDomainAutoTunes</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>DescribeDomainAutoTunes</code> operations, which returns results in the next page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
