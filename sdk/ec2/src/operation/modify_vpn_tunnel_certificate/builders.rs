// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpn_tunnel_certificate::_modify_vpn_tunnel_certificate_output::ModifyVpnTunnelCertificateOutputBuilder;

pub use crate::operation::modify_vpn_tunnel_certificate::_modify_vpn_tunnel_certificate_input::ModifyVpnTunnelCertificateInputBuilder;

impl ModifyVpnTunnelCertificateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_vpn_tunnel_certificate();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVpnTunnelCertificate`.
///
/// <p>Modifies the VPN tunnel endpoint certificate.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVpnTunnelCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ModifyVpnTunnelCertificateFluentBuilder {
    /// Creates a new `ModifyVpnTunnelCertificate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVpnTunnelCertificate as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateInputBuilder {
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
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
            crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError>,
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
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn vpn_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpn_connection_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn set_vpn_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpn_connection_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn get_vpn_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpn_connection_id()
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn vpn_tunnel_outside_ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpn_tunnel_outside_ip_address(input.into());
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn set_vpn_tunnel_outside_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpn_tunnel_outside_ip_address(input);
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn get_vpn_tunnel_outside_ip_address(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpn_tunnel_outside_ip_address()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
