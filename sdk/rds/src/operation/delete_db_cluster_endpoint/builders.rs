// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_db_cluster_endpoint::_delete_db_cluster_endpoint_output::DeleteDbClusterEndpointOutputBuilder;

pub use crate::operation::delete_db_cluster_endpoint::_delete_db_cluster_endpoint_input::DeleteDbClusterEndpointInputBuilder;

impl DeleteDbClusterEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_db_cluster_endpoint::DeleteDbClusterEndpointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_db_cluster_endpoint::DeleteDBClusterEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_db_cluster_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDBClusterEndpoint`.
///
/// <p>Deletes a custom endpoint and removes it from an Amazon Aurora DB cluster.</p> <note>
/// <p>This action only applies to Aurora DB clusters.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDBClusterEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_db_cluster_endpoint::builders::DeleteDbClusterEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteDBClusterEndpointFluentBuilder {
    /// Creates a new `DeleteDBClusterEndpoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDBClusterEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_db_cluster_endpoint::builders::DeleteDbClusterEndpointInputBuilder {
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
        crate::operation::delete_db_cluster_endpoint::DeleteDbClusterEndpointOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_db_cluster_endpoint::DeleteDBClusterEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_db_cluster_endpoint::DeleteDBClusterEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_db_cluster_endpoint::DeleteDBClusterEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_db_cluster_endpoint::DeleteDbClusterEndpointOutput,
            crate::operation::delete_db_cluster_endpoint::DeleteDBClusterEndpointError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_db_cluster_endpoint::DeleteDBClusterEndpointError>,
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
    /// <p>The identifier associated with the custom endpoint. This parameter is stored as a lowercase string.</p>
    pub fn db_cluster_endpoint_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_cluster_endpoint_identifier(input.into());
        self
    }
    /// <p>The identifier associated with the custom endpoint. This parameter is stored as a lowercase string.</p>
    pub fn set_db_cluster_endpoint_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_cluster_endpoint_identifier(input);
        self
    }
    /// <p>The identifier associated with the custom endpoint. This parameter is stored as a lowercase string.</p>
    pub fn get_db_cluster_endpoint_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_cluster_endpoint_identifier()
    }
}
