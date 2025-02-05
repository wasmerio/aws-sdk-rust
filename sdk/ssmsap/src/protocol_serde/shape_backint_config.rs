// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_backint_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::BackintConfig,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.backint_mode {
        object.key("BackintMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ensure_no_backup_in_process {
        object.key("EnsureNoBackupInProcess").boolean(*var_2);
    }
    Ok(())
}
