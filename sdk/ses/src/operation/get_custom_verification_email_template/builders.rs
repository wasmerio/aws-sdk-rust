// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_custom_verification_email_template::_get_custom_verification_email_template_output::GetCustomVerificationEmailTemplateOutputBuilder;

pub use crate::operation::get_custom_verification_email_template::_get_custom_verification_email_template_input::GetCustomVerificationEmailTemplateInputBuilder;

impl GetCustomVerificationEmailTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_custom_verification_email_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetCustomVerificationEmailTemplate`.
///
/// <p>Returns the custom email verification template for the template name you specify.</p>
/// <p>For more information about custom verification email templates, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/custom-verification-emails.html">Using Custom Verification Email Templates</a> in the <i>Amazon SES Developer Guide</i>.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCustomVerificationEmailTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_custom_verification_email_template::builders::GetCustomVerificationEmailTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetCustomVerificationEmailTemplateFluentBuilder {
    /// Creates a new `GetCustomVerificationEmailTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetCustomVerificationEmailTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::get_custom_verification_email_template::builders::GetCustomVerificationEmailTemplateInputBuilder {
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
        crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateOutput,
            crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_custom_verification_email_template::GetCustomVerificationEmailTemplateError>,
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
    /// <p>The name of the custom verification email template that you want to retrieve.</p>
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the custom verification email template that you want to retrieve.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the custom verification email template that you want to retrieve.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_name()
    }
}
