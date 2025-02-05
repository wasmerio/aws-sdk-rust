// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetClientCertificates`](crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`position(impl ::std::convert::Into<String>)`](crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder::position) / [`set_position(Option<String>)`](crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder::set_position): <p>The current pagination position in the paged result set.</p>
    ///   - [`limit(i32)`](crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder::set_limit): <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    /// - On success, responds with [`GetClientCertificatesOutput`](crate::operation::get_client_certificates::GetClientCertificatesOutput) with field(s):
    ///   - [`items(Option<Vec<ClientCertificate>>)`](crate::operation::get_client_certificates::GetClientCertificatesOutput::items): <p>The current page of elements from this collection.</p>
    ///   - [`position(Option<String>)`](crate::operation::get_client_certificates::GetClientCertificatesOutput::position): <p>The current pagination position in the paged result set.</p>
    /// - On failure, responds with [`SdkError<GetClientCertificatesError>`](crate::operation::get_client_certificates::GetClientCertificatesError)
    pub fn get_client_certificates(&self) -> crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder {
        crate::operation::get_client_certificates::builders::GetClientCertificatesFluentBuilder::new(self.handle.clone())
    }
}
