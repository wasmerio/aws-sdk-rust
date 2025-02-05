// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConnectorDefinitionVersion`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connector_definition_id(impl ::std::convert::Into<String>)`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::connector_definition_id) / [`set_connector_definition_id(Option<String>)`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::set_connector_definition_id): The ID of the connector definition.
    ///   - [`connector_definition_version_id(impl ::std::convert::Into<String>)`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::connector_definition_version_id) / [`set_connector_definition_version_id(Option<String>)`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::set_connector_definition_version_id): The ID of the connector definition version. This value maps to the ''Version'' property of the corresponding ''VersionInformation'' object, which is returned by ''ListConnectorDefinitionVersions'' requests. If the version is the last one that was associated with a connector definition, the value also maps to the ''LatestVersion'' property of the corresponding ''DefinitionInformation'' object.
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::set_next_token): The token for the next set of results, or ''null'' if there are no additional results.
    /// - On success, responds with [`GetConnectorDefinitionVersionOutput`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput::arn): The ARN of the connector definition version.
    ///   - [`creation_timestamp(Option<String>)`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the connector definition version was created.
    ///   - [`definition(Option<ConnectorDefinitionVersion>)`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput::definition): Information about the connector definition version.
    ///   - [`id(Option<String>)`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput::id): The ID of the connector definition version.
    ///   - [`next_token(Option<String>)`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput::next_token): The token for the next set of results, or ''null'' if there are no additional results.
    ///   - [`version(Option<String>)`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionOutput::version): The version of the connector definition version.
    /// - On failure, responds with [`SdkError<GetConnectorDefinitionVersionError>`](crate::operation::get_connector_definition_version::GetConnectorDefinitionVersionError)
    pub fn get_connector_definition_version(
        &self,
    ) -> crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder {
        crate::operation::get_connector_definition_version::builders::GetConnectorDefinitionVersionFluentBuilder::new(self.handle.clone())
    }
}
