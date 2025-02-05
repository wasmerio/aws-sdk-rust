// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDNSSEC`](crate::operation::get_dnssec::builders::GetDNSSECFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl ::std::convert::Into<String>)`](crate::operation::get_dnssec::builders::GetDNSSECFluentBuilder::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::operation::get_dnssec::builders::GetDNSSECFluentBuilder::set_hosted_zone_id): <p>A unique string used to identify a hosted zone.</p>
    /// - On success, responds with [`GetDnssecOutput`](crate::operation::get_dnssec::GetDnssecOutput) with field(s):
    ///   - [`status(Option<DnssecStatus>)`](crate::operation::get_dnssec::GetDnssecOutput::status): <p>A string repesenting the status of DNSSEC.</p>
    ///   - [`key_signing_keys(Option<Vec<KeySigningKey>>)`](crate::operation::get_dnssec::GetDnssecOutput::key_signing_keys): <p>The key-signing keys (KSKs) in your account.</p>
    /// - On failure, responds with [`SdkError<GetDNSSECError>`](crate::operation::get_dnssec::GetDNSSECError)
    pub fn get_dnssec(&self) -> crate::operation::get_dnssec::builders::GetDNSSECFluentBuilder {
        crate::operation::get_dnssec::builders::GetDNSSECFluentBuilder::new(self.handle.clone())
    }
}
