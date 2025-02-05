// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpgradePublishedSchema`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`development_schema_arn(impl ::std::convert::Into<String>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::development_schema_arn) / [`set_development_schema_arn(Option<String>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::set_development_schema_arn): <p>The ARN of the development schema with the changes used for the upgrade.</p>
    ///   - [`published_schema_arn(impl ::std::convert::Into<String>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::published_schema_arn) / [`set_published_schema_arn(Option<String>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::set_published_schema_arn): <p>The ARN of the published schema to be upgraded.</p>
    ///   - [`minor_version(impl ::std::convert::Into<String>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::minor_version) / [`set_minor_version(Option<String>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::set_minor_version): <p>Identifies the minor version of the published schema that will be created. This parameter is NOT optional.</p>
    ///   - [`dry_run(bool)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::set_dry_run): <p>Used for testing whether the Development schema provided is backwards compatible, or not, with the publish schema provided by the user to be upgraded. If schema compatibility fails, an exception would be thrown else the call would succeed. This parameter is optional and defaults to false.</p>
    /// - On success, responds with [`UpgradePublishedSchemaOutput`](crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput) with field(s):
    ///   - [`upgraded_schema_arn(Option<String>)`](crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput::upgraded_schema_arn): <p>The ARN of the upgraded schema that is returned as part of the response.</p>
    /// - On failure, responds with [`SdkError<UpgradePublishedSchemaError>`](crate::operation::upgrade_published_schema::UpgradePublishedSchemaError)
    pub fn upgrade_published_schema(&self) -> crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder {
        crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaFluentBuilder::new(self.handle.clone())
    }
}
