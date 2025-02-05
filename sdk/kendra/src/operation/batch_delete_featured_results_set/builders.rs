// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_delete_featured_results_set::_batch_delete_featured_results_set_output::BatchDeleteFeaturedResultsSetOutputBuilder;

pub use crate::operation::batch_delete_featured_results_set::_batch_delete_featured_results_set_input::BatchDeleteFeaturedResultsSetInputBuilder;

impl BatchDeleteFeaturedResultsSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_delete_featured_results_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchDeleteFeaturedResultsSet`.
///
/// <p>Removes one or more sets of featured results. Features results are placed above all other results for certain queries. If there's an exact match of a query, then one or more specific documents are featured in the search results.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDeleteFeaturedResultsSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_delete_featured_results_set::builders::BatchDeleteFeaturedResultsSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl BatchDeleteFeaturedResultsSetFluentBuilder {
    /// Creates a new `BatchDeleteFeaturedResultsSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchDeleteFeaturedResultsSet as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_delete_featured_results_set::builders::BatchDeleteFeaturedResultsSetInputBuilder {
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
        crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetOutput,
            crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::batch_delete_featured_results_set::BatchDeleteFeaturedResultsSetError>,
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
    /// <p>The identifier of the index used for featuring results.</p>
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_id(input.into());
        self
    }
    /// <p>The identifier of the index used for featuring results.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_id(input);
        self
    }
    /// <p>The identifier of the index used for featuring results.</p>
    pub fn get_index_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_index_id()
    }
    /// Appends an item to `FeaturedResultsSetIds`.
    ///
    /// To override the contents of this collection use [`set_featured_results_set_ids`](Self::set_featured_results_set_ids).
    ///
    /// <p>The identifiers of the featured results sets that you want to delete.</p>
    pub fn featured_results_set_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.featured_results_set_ids(input.into());
        self
    }
    /// <p>The identifiers of the featured results sets that you want to delete.</p>
    pub fn set_featured_results_set_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_featured_results_set_ids(input);
        self
    }
    /// <p>The identifiers of the featured results sets that you want to delete.</p>
    pub fn get_featured_results_set_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_featured_results_set_ids()
    }
}
