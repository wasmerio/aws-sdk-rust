// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateNetworkSite`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_site_arn(impl ::std::convert::Into<String>)`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::network_site_arn) / [`set_network_site_arn(Option<String>)`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::set_network_site_arn): <p>The Amazon Resource Name (ARN) of the network site.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::set_description): <p>The description.</p>
    /// - On success, responds with [`UpdateNetworkSiteOutput`](crate::operation::update_network_site::UpdateNetworkSiteOutput) with field(s):
    ///   - [`network_site(Option<NetworkSite>)`](crate::operation::update_network_site::UpdateNetworkSiteOutput::network_site): <p>Information about the network site.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_network_site::UpdateNetworkSiteOutput::tags): <p> The network site tags. </p>
    /// - On failure, responds with [`SdkError<UpdateNetworkSiteError>`](crate::operation::update_network_site::UpdateNetworkSiteError)
    pub fn update_network_site(&self) -> crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder {
        crate::operation::update_network_site::builders::UpdateNetworkSiteFluentBuilder::new(self.handle.clone())
    }
}
