// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_replication_groups::_describe_replication_groups_output::DescribeReplicationGroupsOutputBuilder;

pub use crate::operation::describe_replication_groups::_describe_replication_groups_input::DescribeReplicationGroupsInputBuilder;

impl DescribeReplicationGroupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_replication_groups::DescribeReplicationGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_replication_groups::DescribeReplicationGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_replication_groups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeReplicationGroups`.
///
/// <p>Returns information about a particular replication group. If no identifier is specified, <code>DescribeReplicationGroups</code> returns information about all replication groups.</p> <note>
/// <p>This operation is valid for Redis only.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeReplicationGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_replication_groups::builders::DescribeReplicationGroupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeReplicationGroupsFluentBuilder {
    /// Creates a new `DescribeReplicationGroups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeReplicationGroups as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_replication_groups::builders::DescribeReplicationGroupsInputBuilder {
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
        crate::operation::describe_replication_groups::DescribeReplicationGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_replication_groups::DescribeReplicationGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_replication_groups::DescribeReplicationGroups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_replication_groups::DescribeReplicationGroups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_replication_groups::DescribeReplicationGroupsOutput,
            crate::operation::describe_replication_groups::DescribeReplicationGroupsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_replication_groups::DescribeReplicationGroupsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_replication_groups::paginator::DescribeReplicationGroupsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_replication_groups::paginator::DescribeReplicationGroupsPaginator {
        crate::operation::describe_replication_groups::paginator::DescribeReplicationGroupsPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier for the replication group to be described. This parameter is not case sensitive.</p>
    /// <p>If you do not specify this parameter, information about all replication groups is returned.</p>
    pub fn replication_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_group_id(input.into());
        self
    }
    /// <p>The identifier for the replication group to be described. This parameter is not case sensitive.</p>
    /// <p>If you do not specify this parameter, information about all replication groups is returned.</p>
    pub fn set_replication_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_group_id(input);
        self
    }
    /// <p>The identifier for the replication group to be described. This parameter is not case sensitive.</p>
    /// <p>If you do not specify this parameter, information about all replication groups is returned.</p>
    pub fn get_replication_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_group_id()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: minimum 20; maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: minimum 20; maximum 100.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: minimum 20; maximum 100.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
