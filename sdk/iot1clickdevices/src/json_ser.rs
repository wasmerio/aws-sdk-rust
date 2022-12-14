// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_finalize_device_claim_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::FinalizeDeviceClaimInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.tags {
        #[allow(unused_mut)]
        let mut object_2 = object.key("tags").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_invoke_device_method_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::InvokeDeviceMethodInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.device_method {
        #[allow(unused_mut)]
        let mut object_6 = object.key("deviceMethod").start_object();
        crate::json_ser::serialize_structure_crate_model_device_method(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.device_method_parameters {
        object.key("deviceMethodParameters").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_device_state_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeviceStateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_device_method(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeviceMethod,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.device_type {
        object.key("deviceType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.method_name {
        object.key("methodName").string(var_13.as_str());
    }
    Ok(())
}
