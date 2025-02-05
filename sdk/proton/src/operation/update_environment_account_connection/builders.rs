// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_environment_account_connection::_update_environment_account_connection_output::UpdateEnvironmentAccountConnectionOutputBuilder;

pub use crate::operation::update_environment_account_connection::_update_environment_account_connection_input::UpdateEnvironmentAccountConnectionInputBuilder;

impl UpdateEnvironmentAccountConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_environment_account_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateEnvironmentAccountConnection`.
///
/// <p>In an environment account, update an environment account connection to use a new IAM role.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-env-account-connections.html">Environment account connections</a> in the <i>Proton User guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEnvironmentAccountConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_environment_account_connection::builders::UpdateEnvironmentAccountConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateEnvironmentAccountConnectionFluentBuilder {
    /// Creates a new `UpdateEnvironmentAccountConnection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateEnvironmentAccountConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::update_environment_account_connection::builders::UpdateEnvironmentAccountConnectionInputBuilder {
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
        crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionOutput,
            crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_environment_account_connection::UpdateEnvironmentAccountConnectionError>,
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
    /// <p>The ID of the environment account connection to update.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the environment account connection to update.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the environment account connection to update.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role that's associated with the environment account connection to update.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role that's associated with the environment account connection to update.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role that's associated with the environment account connection to update.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role that Proton uses when provisioning directly defined components in the associated environment account. It determines the scope of infrastructure that a component can provision in the account.</p>
    /// <p>The environment account connection must have a <code>componentRoleArn</code> to allow directly defined components to be associated with any environments running in the account.</p>
    /// <p>For more information about components, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-components.html">Proton components</a> in the <i>Proton User Guide</i>.</p>
    pub fn component_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.component_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role that Proton uses when provisioning directly defined components in the associated environment account. It determines the scope of infrastructure that a component can provision in the account.</p>
    /// <p>The environment account connection must have a <code>componentRoleArn</code> to allow directly defined components to be associated with any environments running in the account.</p>
    /// <p>For more information about components, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-components.html">Proton components</a> in the <i>Proton User Guide</i>.</p>
    pub fn set_component_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_component_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM service role that Proton uses when provisioning directly defined components in the associated environment account. It determines the scope of infrastructure that a component can provision in the account.</p>
    /// <p>The environment account connection must have a <code>componentRoleArn</code> to allow directly defined components to be associated with any environments running in the account.</p>
    /// <p>For more information about components, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-components.html">Proton components</a> in the <i>Proton User Guide</i>.</p>
    pub fn get_component_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_component_role_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM service role in the environment account. Proton uses this role to provision infrastructure resources using CodeBuild-based provisioning in the associated environment account.</p>
    pub fn codebuild_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.codebuild_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM service role in the environment account. Proton uses this role to provision infrastructure resources using CodeBuild-based provisioning in the associated environment account.</p>
    pub fn set_codebuild_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_codebuild_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM service role in the environment account. Proton uses this role to provision infrastructure resources using CodeBuild-based provisioning in the associated environment account.</p>
    pub fn get_codebuild_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_codebuild_role_arn()
    }
}
