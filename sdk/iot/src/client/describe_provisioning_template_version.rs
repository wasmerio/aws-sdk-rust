// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeProvisioningTemplateVersion`](crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_name(impl ::std::convert::Into<String>)`](crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder::template_name) / [`set_template_name(Option<String>)`](crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder::set_template_name): <p>The template name.</p>
    ///   - [`version_id(i32)`](crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder::version_id) / [`set_version_id(Option<i32>)`](crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder::set_version_id): <p>The provisioning template version ID.</p>
    /// - On success, responds with [`DescribeProvisioningTemplateVersionOutput`](crate::operation::describe_provisioning_template_version::DescribeProvisioningTemplateVersionOutput) with field(s):
    ///   - [`version_id(Option<i32>)`](crate::operation::describe_provisioning_template_version::DescribeProvisioningTemplateVersionOutput::version_id): <p>The provisioning template version ID.</p>
    ///   - [`creation_date(Option<DateTime>)`](crate::operation::describe_provisioning_template_version::DescribeProvisioningTemplateVersionOutput::creation_date): <p>The date when the provisioning template version was created.</p>
    ///   - [`template_body(Option<String>)`](crate::operation::describe_provisioning_template_version::DescribeProvisioningTemplateVersionOutput::template_body): <p>The JSON formatted contents of the provisioning template version.</p>
    ///   - [`is_default_version(bool)`](crate::operation::describe_provisioning_template_version::DescribeProvisioningTemplateVersionOutput::is_default_version): <p>True if the provisioning template version is the default version.</p>
    /// - On failure, responds with [`SdkError<DescribeProvisioningTemplateVersionError>`](crate::operation::describe_provisioning_template_version::DescribeProvisioningTemplateVersionError)
    pub fn describe_provisioning_template_version(
        &self,
    ) -> crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder {
        crate::operation::describe_provisioning_template_version::builders::DescribeProvisioningTemplateVersionFluentBuilder::new(self.handle.clone())
    }
}
