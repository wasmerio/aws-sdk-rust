// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::test_repository_triggers::_test_repository_triggers_output::TestRepositoryTriggersOutputBuilder;

pub use crate::operation::test_repository_triggers::_test_repository_triggers_input::TestRepositoryTriggersInputBuilder;

impl TestRepositoryTriggersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::test_repository_triggers::TestRepositoryTriggersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::test_repository_triggers::TestRepositoryTriggersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.test_repository_triggers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TestRepositoryTriggers`.
///
/// <p>Tests the functionality of repository triggers by sending information to the trigger target. If real data is available in the repository, the test sends data from the last commit. If no data is available, sample data is generated.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TestRepositoryTriggersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::test_repository_triggers::builders::TestRepositoryTriggersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl TestRepositoryTriggersFluentBuilder {
    /// Creates a new `TestRepositoryTriggers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TestRepositoryTriggers as a reference.
    pub fn as_input(&self) -> &crate::operation::test_repository_triggers::builders::TestRepositoryTriggersInputBuilder {
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
        crate::operation::test_repository_triggers::TestRepositoryTriggersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::test_repository_triggers::TestRepositoryTriggersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::test_repository_triggers::TestRepositoryTriggers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::test_repository_triggers::TestRepositoryTriggers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::test_repository_triggers::TestRepositoryTriggersOutput,
            crate::operation::test_repository_triggers::TestRepositoryTriggersError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::test_repository_triggers::TestRepositoryTriggersError>,
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
    /// <p>The name of the repository in which to test the triggers.</p>
    pub fn repository_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_name(input.into());
        self
    }
    /// <p>The name of the repository in which to test the triggers.</p>
    pub fn set_repository_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository_name(input);
        self
    }
    /// <p>The name of the repository in which to test the triggers.</p>
    pub fn get_repository_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository_name()
    }
    /// Appends an item to `triggers`.
    ///
    /// To override the contents of this collection use [`set_triggers`](Self::set_triggers).
    ///
    /// <p>The list of triggers to test.</p>
    pub fn triggers(mut self, input: crate::types::RepositoryTrigger) -> Self {
        self.inner = self.inner.triggers(input);
        self
    }
    /// <p>The list of triggers to test.</p>
    pub fn set_triggers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RepositoryTrigger>>) -> Self {
        self.inner = self.inner.set_triggers(input);
        self
    }
    /// <p>The list of triggers to test.</p>
    pub fn get_triggers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RepositoryTrigger>> {
        self.inner.get_triggers()
    }
}
