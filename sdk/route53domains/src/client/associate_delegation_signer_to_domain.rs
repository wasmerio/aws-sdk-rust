// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateDelegationSignerToDomain`](crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder::set_domain_name): <p>The name of the domain.</p>
    ///   - [`signing_attributes(DnssecSigningAttributes)`](crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder::signing_attributes) / [`set_signing_attributes(Option<DnssecSigningAttributes>)`](crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder::set_signing_attributes): <p>The information about a key, including the algorithm, public key-value, and flags.</p>
    /// - On success, responds with [`AssociateDelegationSignerToDomainOutput`](crate::operation::associate_delegation_signer_to_domain::AssociateDelegationSignerToDomainOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::operation::associate_delegation_signer_to_domain::AssociateDelegationSignerToDomainOutput::operation_id): <p>The identifier for tracking the progress of the request. To query the operation status, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_GetOperationDetail.html">GetOperationDetail</a>.</p>
    /// - On failure, responds with [`SdkError<AssociateDelegationSignerToDomainError>`](crate::operation::associate_delegation_signer_to_domain::AssociateDelegationSignerToDomainError)
    pub fn associate_delegation_signer_to_domain(
        &self,
    ) -> crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder {
        crate::operation::associate_delegation_signer_to_domain::builders::AssociateDelegationSignerToDomainFluentBuilder::new(self.handle.clone())
    }
}
