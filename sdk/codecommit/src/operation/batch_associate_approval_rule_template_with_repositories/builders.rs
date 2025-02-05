// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_associate_approval_rule_template_with_repositories::_batch_associate_approval_rule_template_with_repositories_output::BatchAssociateApprovalRuleTemplateWithRepositoriesOutputBuilder;

pub use crate::operation::batch_associate_approval_rule_template_with_repositories::_batch_associate_approval_rule_template_with_repositories_input::BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder;

impl BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_associate_approval_rule_template_with_repositories();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchAssociateApprovalRuleTemplateWithRepositories`.
///
/// <p>Creates an association between an approval rule template and one or more specified repositories. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchAssociateApprovalRuleTemplateWithRepositoriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::batch_associate_approval_rule_template_with_repositories::builders::BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl BatchAssociateApprovalRuleTemplateWithRepositoriesFluentBuilder {
    /// Creates a new `BatchAssociateApprovalRuleTemplateWithRepositories`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchAssociateApprovalRuleTemplateWithRepositories as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_associate_approval_rule_template_with_repositories::builders::BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder{
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
        crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositories::operation_runtime_plugins(
                                    self.handle.runtime_plugins.clone(),
                                    &self.handle.conf,
                                    self.config_override,
                                );
        crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositories::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesOutput,
            crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesError,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_associate_approval_rule_template_with_repositories::BatchAssociateApprovalRuleTemplateWithRepositoriesError,
        >,
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
    /// <p>The name of the template you want to associate with one or more repositories.</p>
    pub fn approval_rule_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.approval_rule_template_name(input.into());
        self
    }
    /// <p>The name of the template you want to associate with one or more repositories.</p>
    pub fn set_approval_rule_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_approval_rule_template_name(input);
        self
    }
    /// <p>The name of the template you want to associate with one or more repositories.</p>
    pub fn get_approval_rule_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_approval_rule_template_name()
    }
    /// Appends an item to `repositoryNames`.
    ///
    /// To override the contents of this collection use [`set_repository_names`](Self::set_repository_names).
    ///
    /// <p>The names of the repositories you want to associate with the template.</p> <note>
    /// <p>The length constraint limit is for each string in the array. The array itself can be empty.</p>
    /// </note>
    pub fn repository_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_names(input.into());
        self
    }
    /// <p>The names of the repositories you want to associate with the template.</p> <note>
    /// <p>The length constraint limit is for each string in the array. The array itself can be empty.</p>
    /// </note>
    pub fn set_repository_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_repository_names(input);
        self
    }
    /// <p>The names of the repositories you want to associate with the template.</p> <note>
    /// <p>The length constraint limit is for each string in the array. The array itself can be empty.</p>
    /// </note>
    pub fn get_repository_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_repository_names()
    }
}
