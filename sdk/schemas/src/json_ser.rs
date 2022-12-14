// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_discoverer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDiscovererInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.cross_account {
        object.key("CrossAccount").boolean(input.cross_account);
    }
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_arn {
        object.key("SourceArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        #[allow(unused_mut)]
        let mut object_4 = object.key("tags").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_registry_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRegistryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.description {
        object.key("Description").string(var_7.as_str());
    }
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

pub fn serialize_structure_crate_input_create_schema_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSchemaInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.content {
        object.key("Content").string(var_12.as_str());
    }
    if let Some(var_13) = &input.description {
        object.key("Description").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tags {
        #[allow(unused_mut)]
        let mut object_15 = object.key("tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    if let Some(var_18) = &input.r#type {
        object.key("Type").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_discovered_schema_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDiscoveredSchemaInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.events {
        let mut array_20 = object.key("Events").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21.as_str());
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.r#type {
        object.key("Type").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.policy {
        object.key("Policy").string(var_23.as_str());
    }
    if let Some(var_24) = &input.revision_id {
        object.key("RevisionId").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.tags {
        #[allow(unused_mut)]
        let mut object_26 = object.key("tags").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27.as_str()).string(value_28.as_str());
            }
        }
        object_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_discoverer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDiscovererInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.cross_account {
        object.key("CrossAccount").boolean(input.cross_account);
    }
    if let Some(var_29) = &input.description {
        object.key("Description").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_registry_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRegistryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.description {
        object.key("Description").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_schema_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSchemaInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_31) = &input.client_token_id {
        object.key("ClientTokenId").string(var_31.as_str());
    }
    if let Some(var_32) = &input.content {
        object.key("Content").string(var_32.as_str());
    }
    if let Some(var_33) = &input.description {
        object.key("Description").string(var_33.as_str());
    }
    if let Some(var_34) = &input.r#type {
        object.key("Type").string(var_34.as_str());
    }
    Ok(())
}
