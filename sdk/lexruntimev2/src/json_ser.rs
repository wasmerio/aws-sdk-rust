// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_put_session_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutSessionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.messages {
        let mut array_2 = object.key("messages").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_message(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.request_attributes {
        #[allow(unused_mut)]
        let mut object_6 = object.key("requestAttributes").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.session_state {
        #[allow(unused_mut)]
        let mut object_10 = object.key("sessionState").start_object();
        crate::json_ser::serialize_structure_crate_model_session_state(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_recognize_text_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RecognizeTextInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.request_attributes {
        #[allow(unused_mut)]
        let mut object_12 = object.key("requestAttributes").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    if let Some(var_15) = &input.session_state {
        #[allow(unused_mut)]
        let mut object_16 = object.key("sessionState").start_object();
        crate::json_ser::serialize_structure_crate_model_session_state(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.text {
        object.key("text").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_message(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Message,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.content {
        object.key("content").string(var_18.as_str());
    }
    if let Some(var_19) = &input.content_type {
        object.key("contentType").string(var_19.as_str());
    }
    if let Some(var_20) = &input.image_response_card {
        #[allow(unused_mut)]
        let mut object_21 = object.key("imageResponseCard").start_object();
        crate::json_ser::serialize_structure_crate_model_image_response_card(
            &mut object_21,
            var_20,
        )?;
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_session_state(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SessionState,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.dialog_action {
        #[allow(unused_mut)]
        let mut object_23 = object.key("dialogAction").start_object();
        crate::json_ser::serialize_structure_crate_model_dialog_action(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.intent {
        #[allow(unused_mut)]
        let mut object_25 = object.key("intent").start_object();
        crate::json_ser::serialize_structure_crate_model_intent(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.active_contexts {
        let mut array_27 = object.key("activeContexts").start_array();
        for item_28 in var_26 {
            {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_active_context(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.session_attributes {
        #[allow(unused_mut)]
        let mut object_31 = object.key("sessionAttributes").start_object();
        for (key_32, value_33) in var_30 {
            {
                object_31.key(key_32.as_str()).string(value_33.as_str());
            }
        }
        object_31.finish();
    }
    if let Some(var_34) = &input.originating_request_id {
        object.key("originatingRequestId").string(var_34.as_str());
    }
    if let Some(var_35) = &input.runtime_hints {
        #[allow(unused_mut)]
        let mut object_36 = object.key("runtimeHints").start_object();
        crate::json_ser::serialize_structure_crate_model_runtime_hints(&mut object_36, var_35)?;
        object_36.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_response_card(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImageResponseCard,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.title {
        object.key("title").string(var_37.as_str());
    }
    if let Some(var_38) = &input.subtitle {
        object.key("subtitle").string(var_38.as_str());
    }
    if let Some(var_39) = &input.image_url {
        object.key("imageUrl").string(var_39.as_str());
    }
    if let Some(var_40) = &input.buttons {
        let mut array_41 = object.key("buttons").start_array();
        for item_42 in var_40 {
            {
                #[allow(unused_mut)]
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_button(&mut object_43, item_42)?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dialog_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DialogAction,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.r#type {
        object.key("type").string(var_44.as_str());
    }
    if let Some(var_45) = &input.slot_to_elicit {
        object.key("slotToElicit").string(var_45.as_str());
    }
    if let Some(var_46) = &input.slot_elicitation_style {
        object.key("slotElicitationStyle").string(var_46.as_str());
    }
    if let Some(var_47) = &input.sub_slot_to_elicit {
        #[allow(unused_mut)]
        let mut object_48 = object.key("subSlotToElicit").start_object();
        crate::json_ser::serialize_structure_crate_model_elicit_sub_slot(&mut object_48, var_47)?;
        object_48.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_intent(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Intent,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_49) = &input.name {
        object.key("name").string(var_49.as_str());
    }
    if let Some(var_50) = &input.slots {
        #[allow(unused_mut)]
        let mut object_51 = object.key("slots").start_object();
        for (key_52, value_53) in var_50 {
            {
                #[allow(unused_mut)]
                let mut object_54 = object_51.key(key_52.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_slot(&mut object_54, value_53)?;
                object_54.finish();
            }
        }
        object_51.finish();
    }
    if let Some(var_55) = &input.state {
        object.key("state").string(var_55.as_str());
    }
    if let Some(var_56) = &input.confirmation_state {
        object.key("confirmationState").string(var_56.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_context(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActiveContext,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.name {
        object.key("name").string(var_57.as_str());
    }
    if let Some(var_58) = &input.time_to_live {
        #[allow(unused_mut)]
        let mut object_59 = object.key("timeToLive").start_object();
        crate::json_ser::serialize_structure_crate_model_active_context_time_to_live(
            &mut object_59,
            var_58,
        )?;
        object_59.finish();
    }
    if let Some(var_60) = &input.context_attributes {
        #[allow(unused_mut)]
        let mut object_61 = object.key("contextAttributes").start_object();
        for (key_62, value_63) in var_60 {
            {
                object_61.key(key_62.as_str()).string(value_63.as_str());
            }
        }
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_runtime_hints(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RuntimeHints,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_64) = &input.slot_hints {
        #[allow(unused_mut)]
        let mut object_65 = object.key("slotHints").start_object();
        for (key_66, value_67) in var_64 {
            {
                #[allow(unused_mut)]
                let mut object_68 = object_65.key(key_66.as_str()).start_object();
                for (key_69, value_70) in value_67 {
                    {
                        #[allow(unused_mut)]
                        let mut object_71 = object_68.key(key_69.as_str()).start_object();
                        crate::json_ser::serialize_structure_crate_model_runtime_hint_details(
                            &mut object_71,
                            value_70,
                        )?;
                        object_71.finish();
                    }
                }
                object_68.finish();
            }
        }
        object_65.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_button(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Button,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.text {
        object.key("text").string(var_72.as_str());
    }
    if let Some(var_73) = &input.value {
        object.key("value").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_elicit_sub_slot(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ElicitSubSlot,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.name {
        object.key("name").string(var_74.as_str());
    }
    if let Some(var_75) = &input.sub_slot_to_elicit {
        #[allow(unused_mut)]
        let mut object_76 = object.key("subSlotToElicit").start_object();
        crate::json_ser::serialize_structure_crate_model_elicit_sub_slot(&mut object_76, var_75)?;
        object_76.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_slot(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Slot,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_77) = &input.value {
        #[allow(unused_mut)]
        let mut object_78 = object.key("value").start_object();
        crate::json_ser::serialize_structure_crate_model_value(&mut object_78, var_77)?;
        object_78.finish();
    }
    if let Some(var_79) = &input.shape {
        object.key("shape").string(var_79.as_str());
    }
    if let Some(var_80) = &input.values {
        let mut array_81 = object.key("values").start_array();
        for item_82 in var_80 {
            {
                #[allow(unused_mut)]
                let mut object_83 = array_81.value().start_object();
                crate::json_ser::serialize_structure_crate_model_slot(&mut object_83, item_82)?;
                object_83.finish();
            }
        }
        array_81.finish();
    }
    if let Some(var_84) = &input.sub_slots {
        #[allow(unused_mut)]
        let mut object_85 = object.key("subSlots").start_object();
        for (key_86, value_87) in var_84 {
            {
                #[allow(unused_mut)]
                let mut object_88 = object_85.key(key_86.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_slot(&mut object_88, value_87)?;
                object_88.finish();
            }
        }
        object_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_context_time_to_live(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActiveContextTimeToLive,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.time_to_live_in_seconds {
        object.key("timeToLiveInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_89).into()),
        );
    }
    if let Some(var_90) = &input.turns_to_live {
        object.key("turnsToLive").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_90).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_runtime_hint_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RuntimeHintDetails,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.runtime_hint_values {
        let mut array_92 = object.key("runtimeHintValues").start_array();
        for item_93 in var_91 {
            {
                #[allow(unused_mut)]
                let mut object_94 = array_92.value().start_object();
                crate::json_ser::serialize_structure_crate_model_runtime_hint_value(
                    &mut object_94,
                    item_93,
                )?;
                object_94.finish();
            }
        }
        array_92.finish();
    }
    if let Some(var_95) = &input.sub_slot_hints {
        #[allow(unused_mut)]
        let mut object_96 = object.key("subSlotHints").start_object();
        for (key_97, value_98) in var_95 {
            {
                #[allow(unused_mut)]
                let mut object_99 = object_96.key(key_97.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_runtime_hint_details(
                    &mut object_99,
                    value_98,
                )?;
                object_99.finish();
            }
        }
        object_96.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Value,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.original_value {
        object.key("originalValue").string(var_100.as_str());
    }
    if let Some(var_101) = &input.interpreted_value {
        object.key("interpretedValue").string(var_101.as_str());
    }
    if let Some(var_102) = &input.resolved_values {
        let mut array_103 = object.key("resolvedValues").start_array();
        for item_104 in var_102 {
            {
                array_103.value().string(item_104.as_str());
            }
        }
        array_103.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_runtime_hint_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RuntimeHintValue,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_105) = &input.phrase {
        object.key("phrase").string(var_105.as_str());
    }
    Ok(())
}
