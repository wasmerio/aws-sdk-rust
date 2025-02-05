// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetServiceQuotaIncreaseRequestFromTemplate`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_code(impl ::std::convert::Into<String>)`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::service_code) / [`set_service_code(Option<String>)`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::set_service_code): <p>The service identifier.</p>
    ///   - [`quota_code(impl ::std::convert::Into<String>)`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::quota_code) / [`set_quota_code(Option<String>)`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::set_quota_code): <p>The quota identifier.</p>
    ///   - [`aws_region(impl ::std::convert::Into<String>)`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::aws_region) / [`set_aws_region(Option<String>)`](crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::set_aws_region): <p>The AWS Region.</p>
    /// - On success, responds with [`GetServiceQuotaIncreaseRequestFromTemplateOutput`](crate::operation::get_service_quota_increase_request_from_template::GetServiceQuotaIncreaseRequestFromTemplateOutput) with field(s):
    ///   - [`service_quota_increase_request_in_template(Option<ServiceQuotaIncreaseRequestInTemplate>)`](crate::operation::get_service_quota_increase_request_from_template::GetServiceQuotaIncreaseRequestFromTemplateOutput::service_quota_increase_request_in_template): <p>Information about the quota increase request.</p>
    /// - On failure, responds with [`SdkError<GetServiceQuotaIncreaseRequestFromTemplateError>`](crate::operation::get_service_quota_increase_request_from_template::GetServiceQuotaIncreaseRequestFromTemplateError)
    pub fn get_service_quota_increase_request_from_template(
        &self,
    ) -> crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder {
        crate::operation::get_service_quota_increase_request_from_template::builders::GetServiceQuotaIncreaseRequestFromTemplateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
