// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_budget(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Budget,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.budget_name {
        object.key("BudgetName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.budget_limit {
        #[allow(unused_mut)]
        let mut object_3 = object.key("BudgetLimit").start_object();
        crate::protocol_serde::shape_spend::ser_spend(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.planned_budget_limits {
        #[allow(unused_mut)]
        let mut object_5 = object.key("PlannedBudgetLimits").start_object();
        for (key_6, value_7) in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_8 = object_5.key(key_6.as_str()).start_object();
                crate::protocol_serde::shape_spend::ser_spend(&mut object_8, value_7)?;
                object_8.finish();
            }
        }
        object_5.finish();
    }
    if let Some(var_9) = &input.cost_filters {
        #[allow(unused_mut)]
        let mut object_10 = object.key("CostFilters").start_object();
        for (key_11, value_12) in var_9 {
            {
                let mut array_13 = object_10.key(key_11.as_str()).start_array();
                for item_14 in value_12 {
                    {
                        array_13.value().string(item_14.as_str());
                    }
                }
                array_13.finish();
            }
        }
        object_10.finish();
    }
    if let Some(var_15) = &input.cost_types {
        #[allow(unused_mut)]
        let mut object_16 = object.key("CostTypes").start_object();
        crate::protocol_serde::shape_cost_types::ser_cost_types(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.time_unit {
        object.key("TimeUnit").string(var_17.as_str());
    }
    if let Some(var_18) = &input.time_period {
        #[allow(unused_mut)]
        let mut object_19 = object.key("TimePeriod").start_object();
        crate::protocol_serde::shape_time_period::ser_time_period(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.calculated_spend {
        #[allow(unused_mut)]
        let mut object_21 = object.key("CalculatedSpend").start_object();
        crate::protocol_serde::shape_calculated_spend::ser_calculated_spend(&mut object_21, var_20)?;
        object_21.finish();
    }
    if let Some(var_22) = &input.budget_type {
        object.key("BudgetType").string(var_22.as_str());
    }
    if let Some(var_23) = &input.last_updated_time {
        object
            .key("LastUpdatedTime")
            .date_time(var_23, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_24) = &input.auto_adjust_data {
        #[allow(unused_mut)]
        let mut object_25 = object.key("AutoAdjustData").start_object();
        crate::protocol_serde::shape_auto_adjust_data::ser_auto_adjust_data(&mut object_25, var_24)?;
        object_25.finish();
    }
    Ok(())
}

pub(crate) fn de_budget<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::Budget>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::BudgetBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "BudgetName" => {
                            builder = builder.set_budget_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "BudgetLimit" => {
                            builder = builder.set_budget_limit(crate::protocol_serde::shape_spend::de_spend(tokens)?);
                        }
                        "PlannedBudgetLimits" => {
                            builder = builder
                                .set_planned_budget_limits(crate::protocol_serde::shape_planned_budget_limits::de_planned_budget_limits(tokens)?);
                        }
                        "CostFilters" => {
                            builder = builder.set_cost_filters(crate::protocol_serde::shape_cost_filters::de_cost_filters(tokens)?);
                        }
                        "CostTypes" => {
                            builder = builder.set_cost_types(crate::protocol_serde::shape_cost_types::de_cost_types(tokens)?);
                        }
                        "TimeUnit" => {
                            builder = builder.set_time_unit(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::TimeUnit::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "TimePeriod" => {
                            builder = builder.set_time_period(crate::protocol_serde::shape_time_period::de_time_period(tokens)?);
                        }
                        "CalculatedSpend" => {
                            builder = builder.set_calculated_spend(crate::protocol_serde::shape_calculated_spend::de_calculated_spend(tokens)?);
                        }
                        "BudgetType" => {
                            builder = builder.set_budget_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::BudgetType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LastUpdatedTime" => {
                            builder = builder.set_last_updated_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "AutoAdjustData" => {
                            builder = builder.set_auto_adjust_data(crate::protocol_serde::shape_auto_adjust_data::de_auto_adjust_data(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
