// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePublicDnsNamespace`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::set_id): <p>The ID of the namespace being updated.</p>
    ///   - [`updater_request_id(impl ::std::convert::Into<String>)`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::updater_request_id) / [`set_updater_request_id(Option<String>)`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::set_updater_request_id): <p>A unique string that identifies the request and that allows failed <code>UpdatePublicDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    ///   - [`namespace(PublicDnsNamespaceChange)`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::namespace) / [`set_namespace(Option<PublicDnsNamespaceChange>)`](crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::set_namespace): <p>Updated properties for the public DNS namespace.</p>
    /// - On success, responds with [`UpdatePublicDnsNamespaceOutput`](crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceOutput::operation_id): <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    /// - On failure, responds with [`SdkError<UpdatePublicDnsNamespaceError>`](crate::operation::update_public_dns_namespace::UpdatePublicDnsNamespaceError)
    pub fn update_public_dns_namespace(&self) -> crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder {
        crate::operation::update_public_dns_namespace::builders::UpdatePublicDnsNamespaceFluentBuilder::new(self.handle.clone())
    }
}
