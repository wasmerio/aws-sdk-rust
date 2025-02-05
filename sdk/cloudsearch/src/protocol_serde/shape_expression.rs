// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_expression(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::Expression,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ExpressionName");
    if let Some(var_2) = &input.expression_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ExpressionValue");
    if let Some(var_4) = &input.expression_value {
        scope_3.string(var_4);
    }
    Ok(())
}

pub fn de_expression(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Expression, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Expression::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ExpressionName") /* ExpressionName com.amazonaws.cloudsearch#Expression$ExpressionName */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_expression_name(var_5);
            }
            ,
            s if s.matches("ExpressionValue") /* ExpressionValue com.amazonaws.cloudsearch#Expression$ExpressionValue */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_expression_value(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
