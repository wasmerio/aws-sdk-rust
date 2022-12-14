// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_domain_transfer_from_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptDomainTransferFromAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.password {
        object.key("Password").string(var_2.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_cancel_domain_transfer_to_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelDomainTransferToAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_3) = &input.domain_name {
        object.key("DomainName").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_check_domain_availability_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CheckDomainAvailabilityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.domain_name {
        object.key("DomainName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.idn_lang_code {
        object.key("IdnLangCode").string(var_5.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_check_domain_transferability_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CheckDomainTransferabilityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.domain_name {
        object.key("DomainName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.auth_code {
        object.key("AuthCode").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.domain_name {
        object.key("DomainName").string(var_8.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_tags_for_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTagsForDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.domain_name {
        object.key("DomainName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags_to_delete {
        let mut array_11 = object.key("TagsToDelete").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_domain_auto_renew_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableDomainAutoRenewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_13) = &input.domain_name {
        object.key("DomainName").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_domain_transfer_lock_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableDomainTransferLockInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.domain_name {
        object.key("DomainName").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_domain_auto_renew_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableDomainAutoRenewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.domain_name {
        object.key("DomainName").string(var_15.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_domain_transfer_lock_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableDomainTransferLockInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.domain_name {
        object.key("DomainName").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_contact_reachability_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetContactReachabilityStatusInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.domain_name {
        object.key("domainName").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_domain_detail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDomainDetailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.domain_name {
        object.key("DomainName").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_domain_suggestions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDomainSuggestionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.domain_name {
        object.key("DomainName").string(var_19.as_str());
    }
    {
        object.key("SuggestionCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.suggestion_count).into()),
        );
    }
    if let Some(var_20) = &input.only_available {
        object.key("OnlyAvailable").boolean(*var_20);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_operation_detail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetOperationDetailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.operation_id {
        object.key("OperationId").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_domains_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDomainsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.filter_conditions {
        let mut array_23 = object.key("FilterConditions").start_array();
        for item_24 in var_22 {
            {
                #[allow(unused_mut)]
                let mut object_25 = array_23.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_condition(
                    &mut object_25,
                    item_24,
                )?;
                object_25.finish();
            }
        }
        array_23.finish();
    }
    if let Some(var_26) = &input.sort_condition {
        #[allow(unused_mut)]
        let mut object_27 = object.key("SortCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_sort_condition(&mut object_27, var_26)?;
        object_27.finish();
    }
    if let Some(var_28) = &input.marker {
        object.key("Marker").string(var_28.as_str());
    }
    if let Some(var_29) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_29).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_operations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListOperationsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.submitted_since {
        object
            .key("SubmittedSince")
            .date_time(var_30, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_31) = &input.marker {
        object.key("Marker").string(var_31.as_str());
    }
    if let Some(var_32) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_prices_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPricesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.tld {
        object.key("Tld").string(var_33.as_str());
    }
    if let Some(var_34) = &input.marker {
        object.key("Marker").string(var_34.as_str());
    }
    if let Some(var_35) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_35).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.domain_name {
        object.key("DomainName").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.domain_name {
        object.key("DomainName").string(var_37.as_str());
    }
    if let Some(var_38) = &input.idn_lang_code {
        object.key("IdnLangCode").string(var_38.as_str());
    }
    if let Some(var_39) = &input.duration_in_years {
        object.key("DurationInYears").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
    if let Some(var_40) = &input.auto_renew {
        object.key("AutoRenew").boolean(*var_40);
    }
    if let Some(var_41) = &input.admin_contact {
        #[allow(unused_mut)]
        let mut object_42 = object.key("AdminContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_42, var_41)?;
        object_42.finish();
    }
    if let Some(var_43) = &input.registrant_contact {
        #[allow(unused_mut)]
        let mut object_44 = object.key("RegistrantContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_44, var_43)?;
        object_44.finish();
    }
    if let Some(var_45) = &input.tech_contact {
        #[allow(unused_mut)]
        let mut object_46 = object.key("TechContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_46, var_45)?;
        object_46.finish();
    }
    if let Some(var_47) = &input.privacy_protect_admin_contact {
        object.key("PrivacyProtectAdminContact").boolean(*var_47);
    }
    if let Some(var_48) = &input.privacy_protect_registrant_contact {
        object
            .key("PrivacyProtectRegistrantContact")
            .boolean(*var_48);
    }
    if let Some(var_49) = &input.privacy_protect_tech_contact {
        object.key("PrivacyProtectTechContact").boolean(*var_49);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reject_domain_transfer_from_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectDomainTransferFromAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.domain_name {
        object.key("DomainName").string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_renew_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RenewDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_51) = &input.domain_name {
        object.key("DomainName").string(var_51.as_str());
    }
    if let Some(var_52) = &input.duration_in_years {
        object.key("DurationInYears").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_52).into()),
        );
    }
    {
        object.key("CurrentExpiryYear").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.current_expiry_year).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resend_contact_reachability_email_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResendContactReachabilityEmailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.domain_name {
        object.key("domainName").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_retrieve_domain_auth_code_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RetrieveDomainAuthCodeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_54) = &input.domain_name {
        object.key("DomainName").string(var_54.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_transfer_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TransferDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_55) = &input.domain_name {
        object.key("DomainName").string(var_55.as_str());
    }
    if let Some(var_56) = &input.idn_lang_code {
        object.key("IdnLangCode").string(var_56.as_str());
    }
    if let Some(var_57) = &input.duration_in_years {
        object.key("DurationInYears").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.nameservers {
        let mut array_59 = object.key("Nameservers").start_array();
        for item_60 in var_58 {
            {
                #[allow(unused_mut)]
                let mut object_61 = array_59.value().start_object();
                crate::json_ser::serialize_structure_crate_model_nameserver(
                    &mut object_61,
                    item_60,
                )?;
                object_61.finish();
            }
        }
        array_59.finish();
    }
    if let Some(var_62) = &input.auth_code {
        object.key("AuthCode").string(var_62.as_str());
    }
    if let Some(var_63) = &input.auto_renew {
        object.key("AutoRenew").boolean(*var_63);
    }
    if let Some(var_64) = &input.admin_contact {
        #[allow(unused_mut)]
        let mut object_65 = object.key("AdminContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_65, var_64)?;
        object_65.finish();
    }
    if let Some(var_66) = &input.registrant_contact {
        #[allow(unused_mut)]
        let mut object_67 = object.key("RegistrantContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_67, var_66)?;
        object_67.finish();
    }
    if let Some(var_68) = &input.tech_contact {
        #[allow(unused_mut)]
        let mut object_69 = object.key("TechContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_69, var_68)?;
        object_69.finish();
    }
    if let Some(var_70) = &input.privacy_protect_admin_contact {
        object.key("PrivacyProtectAdminContact").boolean(*var_70);
    }
    if let Some(var_71) = &input.privacy_protect_registrant_contact {
        object
            .key("PrivacyProtectRegistrantContact")
            .boolean(*var_71);
    }
    if let Some(var_72) = &input.privacy_protect_tech_contact {
        object.key("PrivacyProtectTechContact").boolean(*var_72);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_transfer_domain_to_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TransferDomainToAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.domain_name {
        object.key("DomainName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.account_id {
        object.key("AccountId").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_contact_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainContactInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_75) = &input.domain_name {
        object.key("DomainName").string(var_75.as_str());
    }
    if let Some(var_76) = &input.admin_contact {
        #[allow(unused_mut)]
        let mut object_77 = object.key("AdminContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_77, var_76)?;
        object_77.finish();
    }
    if let Some(var_78) = &input.registrant_contact {
        #[allow(unused_mut)]
        let mut object_79 = object.key("RegistrantContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_79, var_78)?;
        object_79.finish();
    }
    if let Some(var_80) = &input.tech_contact {
        #[allow(unused_mut)]
        let mut object_81 = object.key("TechContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_81, var_80)?;
        object_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_contact_privacy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainContactPrivacyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.domain_name {
        object.key("DomainName").string(var_82.as_str());
    }
    if let Some(var_83) = &input.admin_privacy {
        object.key("AdminPrivacy").boolean(*var_83);
    }
    if let Some(var_84) = &input.registrant_privacy {
        object.key("RegistrantPrivacy").boolean(*var_84);
    }
    if let Some(var_85) = &input.tech_privacy {
        object.key("TechPrivacy").boolean(*var_85);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_nameservers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainNameserversInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.domain_name {
        object.key("DomainName").string(var_86.as_str());
    }
    if let Some(var_87) = &input.fi_auth_key {
        object.key("FIAuthKey").string(var_87.as_str());
    }
    if let Some(var_88) = &input.nameservers {
        let mut array_89 = object.key("Nameservers").start_array();
        for item_90 in var_88 {
            {
                #[allow(unused_mut)]
                let mut object_91 = array_89.value().start_object();
                crate::json_ser::serialize_structure_crate_model_nameserver(
                    &mut object_91,
                    item_90,
                )?;
                object_91.finish();
            }
        }
        array_89.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_tags_for_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTagsForDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.domain_name {
        object.key("DomainName").string(var_92.as_str());
    }
    if let Some(var_93) = &input.tags_to_update {
        let mut array_94 = object.key("TagsToUpdate").start_array();
        for item_95 in var_93 {
            {
                #[allow(unused_mut)]
                let mut object_96 = array_94.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_96, item_95)?;
                object_96.finish();
            }
        }
        array_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_view_billing_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ViewBillingInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.start {
        object
            .key("Start")
            .date_time(var_97, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_98) = &input.end {
        object
            .key("End")
            .date_time(var_98, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_99) = &input.marker {
        object.key("Marker").string(var_99.as_str());
    }
    if let Some(var_100) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_100).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FilterCondition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_101) = &input.name {
        object.key("Name").string(var_101.as_str());
    }
    if let Some(var_102) = &input.operator {
        object.key("Operator").string(var_102.as_str());
    }
    if let Some(var_103) = &input.values {
        let mut array_104 = object.key("Values").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sort_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SortCondition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_106) = &input.name {
        object.key("Name").string(var_106.as_str());
    }
    if let Some(var_107) = &input.sort_order {
        object.key("SortOrder").string(var_107.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_contact_detail(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ContactDetail,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_108) = &input.first_name {
        object.key("FirstName").string(var_108.as_str());
    }
    if let Some(var_109) = &input.last_name {
        object.key("LastName").string(var_109.as_str());
    }
    if let Some(var_110) = &input.contact_type {
        object.key("ContactType").string(var_110.as_str());
    }
    if let Some(var_111) = &input.organization_name {
        object.key("OrganizationName").string(var_111.as_str());
    }
    if let Some(var_112) = &input.address_line1 {
        object.key("AddressLine1").string(var_112.as_str());
    }
    if let Some(var_113) = &input.address_line2 {
        object.key("AddressLine2").string(var_113.as_str());
    }
    if let Some(var_114) = &input.city {
        object.key("City").string(var_114.as_str());
    }
    if let Some(var_115) = &input.state {
        object.key("State").string(var_115.as_str());
    }
    if let Some(var_116) = &input.country_code {
        object.key("CountryCode").string(var_116.as_str());
    }
    if let Some(var_117) = &input.zip_code {
        object.key("ZipCode").string(var_117.as_str());
    }
    if let Some(var_118) = &input.phone_number {
        object.key("PhoneNumber").string(var_118.as_str());
    }
    if let Some(var_119) = &input.email {
        object.key("Email").string(var_119.as_str());
    }
    if let Some(var_120) = &input.fax {
        object.key("Fax").string(var_120.as_str());
    }
    if let Some(var_121) = &input.extra_params {
        let mut array_122 = object.key("ExtraParams").start_array();
        for item_123 in var_121 {
            {
                #[allow(unused_mut)]
                let mut object_124 = array_122.value().start_object();
                crate::json_ser::serialize_structure_crate_model_extra_param(
                    &mut object_124,
                    item_123,
                )?;
                object_124.finish();
            }
        }
        array_122.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_nameserver(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Nameserver,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_125) = &input.name {
        object.key("Name").string(var_125.as_str());
    }
    if let Some(var_126) = &input.glue_ips {
        let mut array_127 = object.key("GlueIps").start_array();
        for item_128 in var_126 {
            {
                array_127.value().string(item_128.as_str());
            }
        }
        array_127.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.key {
        object.key("Key").string(var_129.as_str());
    }
    if let Some(var_130) = &input.value {
        object.key("Value").string(var_130.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_extra_param(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExtraParam,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_131) = &input.name {
        object.key("Name").string(var_131.as_str());
    }
    if let Some(var_132) = &input.value {
        object.key("Value").string(var_132.as_str());
    }
    Ok(())
}
