// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_website_certificate_authority::_describe_website_certificate_authority_output::DescribeWebsiteCertificateAuthorityOutputBuilder;

pub use crate::operation::describe_website_certificate_authority::_describe_website_certificate_authority_input::DescribeWebsiteCertificateAuthorityInputBuilder;

impl DescribeWebsiteCertificateAuthorityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_website_certificate_authority();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeWebsiteCertificateAuthority`.
///
/// <p>Provides information about the certificate authority.</p>
#[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeWebsiteCertificateAuthorityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_website_certificate_authority::builders::DescribeWebsiteCertificateAuthorityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeWebsiteCertificateAuthorityFluentBuilder {
    /// Creates a new `DescribeWebsiteCertificateAuthority`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeWebsiteCertificateAuthority as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_website_certificate_authority::builders::DescribeWebsiteCertificateAuthorityInputBuilder {
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
        crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthority::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthority::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityOutput,
            crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_website_certificate_authority::DescribeWebsiteCertificateAuthorityError>,
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
    /// <p>The ARN of the fleet.</p>
    pub fn fleet_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_arn(input.into());
        self
    }
    /// <p>The ARN of the fleet.</p>
    pub fn set_fleet_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_arn(input);
        self
    }
    /// <p>The ARN of the fleet.</p>
    pub fn get_fleet_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_arn()
    }
    /// <p>A unique identifier for the certificate authority.</p>
    pub fn website_ca_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.website_ca_id(input.into());
        self
    }
    /// <p>A unique identifier for the certificate authority.</p>
    pub fn set_website_ca_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_website_ca_id(input);
        self
    }
    /// <p>A unique identifier for the certificate authority.</p>
    pub fn get_website_ca_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_website_ca_id()
    }
}
