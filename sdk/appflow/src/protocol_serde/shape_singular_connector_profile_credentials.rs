// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_singular_connector_profile_credentials(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SingularConnectorProfileCredentials,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_key {
        object.key("apiKey").string(var_1.as_str());
    }
    Ok(())
}
