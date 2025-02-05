// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_availability_configuration::_create_availability_configuration_output::CreateAvailabilityConfigurationOutputBuilder;

pub use crate::operation::create_availability_configuration::_create_availability_configuration_input::CreateAvailabilityConfigurationInputBuilder;

impl CreateAvailabilityConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_availability_configuration::CreateAvailabilityConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_availability_configuration::CreateAvailabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_availability_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAvailabilityConfiguration`.
///
/// <p>Creates an <code>AvailabilityConfiguration</code> for the given WorkMail organization and domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAvailabilityConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_availability_configuration::builders::CreateAvailabilityConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateAvailabilityConfigurationFluentBuilder {
    /// Creates a new `CreateAvailabilityConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAvailabilityConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::create_availability_configuration::builders::CreateAvailabilityConfigurationInputBuilder {
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
        crate::operation::create_availability_configuration::CreateAvailabilityConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_availability_configuration::CreateAvailabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_availability_configuration::CreateAvailabilityConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_availability_configuration::CreateAvailabilityConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_availability_configuration::CreateAvailabilityConfigurationOutput,
            crate::operation::create_availability_configuration::CreateAvailabilityConfigurationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_availability_configuration::CreateAvailabilityConfigurationError>,
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
    /// <p>An idempotent token that ensures that an API request is executed only once.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>An idempotent token that ensures that an API request is executed only once.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>An idempotent token that ensures that an API request is executed only once.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be created.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be created.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be created.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_id()
    }
    /// <p>The domain to which the provider applies.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The domain to which the provider applies.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The domain to which the provider applies.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>Exchange Web Services (EWS) availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p>
    pub fn ews_provider(mut self, input: crate::types::EwsAvailabilityProvider) -> Self {
        self.inner = self.inner.ews_provider(input);
        self
    }
    /// <p>Exchange Web Services (EWS) availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p>
    pub fn set_ews_provider(mut self, input: ::std::option::Option<crate::types::EwsAvailabilityProvider>) -> Self {
        self.inner = self.inner.set_ews_provider(input);
        self
    }
    /// <p>Exchange Web Services (EWS) availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p>
    pub fn get_ews_provider(&self) -> &::std::option::Option<crate::types::EwsAvailabilityProvider> {
        self.inner.get_ews_provider()
    }
    /// <p>Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p>
    pub fn lambda_provider(mut self, input: crate::types::LambdaAvailabilityProvider) -> Self {
        self.inner = self.inner.lambda_provider(input);
        self
    }
    /// <p>Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p>
    pub fn set_lambda_provider(mut self, input: ::std::option::Option<crate::types::LambdaAvailabilityProvider>) -> Self {
        self.inner = self.inner.set_lambda_provider(input);
        self
    }
    /// <p>Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p>
    pub fn get_lambda_provider(&self) -> &::std::option::Option<crate::types::LambdaAvailabilityProvider> {
        self.inner.get_lambda_provider()
    }
}
