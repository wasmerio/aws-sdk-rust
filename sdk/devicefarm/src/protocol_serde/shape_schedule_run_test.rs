// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_schedule_run_test(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ScheduleRunTest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.r#type {
        object.key("type").string(var_1.as_str());
    }
    if let Some(var_2) = &input.test_package_arn {
        object.key("testPackageArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.test_spec_arn {
        object.key("testSpecArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.filter {
        object.key("filter").string(var_4.as_str());
    }
    if let Some(var_5) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_6 = object.key("parameters").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    Ok(())
}
