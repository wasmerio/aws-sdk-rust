// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TransferCertificate`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_id(impl ::std::convert::Into<String>)`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::certificate_id) / [`set_certificate_id(Option<String>)`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::set_certificate_id): <p>The ID of the certificate. (The last part of the certificate ARN contains the certificate ID.)</p>
    ///   - [`target_aws_account(impl ::std::convert::Into<String>)`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::target_aws_account) / [`set_target_aws_account(Option<String>)`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::set_target_aws_account): <p>The Amazon Web Services account.</p>
    ///   - [`transfer_message(impl ::std::convert::Into<String>)`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::transfer_message) / [`set_transfer_message(Option<String>)`](crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::set_transfer_message): <p>The transfer message.</p>
    /// - On success, responds with [`TransferCertificateOutput`](crate::operation::transfer_certificate::TransferCertificateOutput) with field(s):
    ///   - [`transferred_certificate_arn(Option<String>)`](crate::operation::transfer_certificate::TransferCertificateOutput::transferred_certificate_arn): <p>The ARN of the certificate.</p>
    /// - On failure, responds with [`SdkError<TransferCertificateError>`](crate::operation::transfer_certificate::TransferCertificateError)
    pub fn transfer_certificate(&self) -> crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder {
        crate::operation::transfer_certificate::builders::TransferCertificateFluentBuilder::new(self.handle.clone())
    }
}
