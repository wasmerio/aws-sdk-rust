// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_total_impact_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TotalImpactFilter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.numeric_operator {
        object.key("NumericOperator").string(var_1.as_str());
    }
    {
        object.key("StartValue").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.start_value).into()),
        );
    }
    if input.end_value != 0.0 {
        object.key("EndValue").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.end_value).into()),
        );
    }
    Ok(())
}
