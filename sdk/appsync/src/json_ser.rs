// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_api_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateApiInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_id {
        object.key("apiId").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_api_cache_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApiCacheInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.api_caching_behavior {
        object.key("apiCachingBehavior").string(var_2.as_str());
    }
    if input.at_rest_encryption_enabled {
        object
            .key("atRestEncryptionEnabled")
            .boolean(input.at_rest_encryption_enabled);
    }
    if input.transit_encryption_enabled {
        object
            .key("transitEncryptionEnabled")
            .boolean(input.transit_encryption_enabled);
    }
    {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    if let Some(var_3) = &input.r#type {
        object.key("type").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_api_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApiKeyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if input.expires != 0 {
        object.key("expires").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.expires).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_data_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataSourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.dynamodb_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("dynamodbConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dynamodb_data_source_config(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    if let Some(var_8) = &input.elasticsearch_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("elasticsearchConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_elasticsearch_data_source_config(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.http_config {
        #[allow(unused_mut)]
        let mut object_11 = object.key("httpConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_http_data_source_config(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.lambda_config {
        #[allow(unused_mut)]
        let mut object_13 = object.key("lambdaConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_data_source_config(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.name {
        object.key("name").string(var_14.as_str());
    }
    if let Some(var_15) = &input.open_search_service_config {
        #[allow(unused_mut)]
        let mut object_16 = object.key("openSearchServiceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_open_search_service_data_source_config(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.relational_database_config {
        #[allow(unused_mut)]
        let mut object_18 = object.key("relationalDatabaseConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_relational_database_data_source_config(
            &mut object_18,
            var_17,
        )?;
        object_18.finish();
    }
    if let Some(var_19) = &input.service_role_arn {
        object.key("serviceRoleArn").string(var_19.as_str());
    }
    if let Some(var_20) = &input.r#type {
        object.key("type").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_domain_name_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDomainNameInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.certificate_arn {
        object.key("certificateArn").string(var_21.as_str());
    }
    if let Some(var_22) = &input.description {
        object.key("description").string(var_22.as_str());
    }
    if let Some(var_23) = &input.domain_name {
        object.key("domainName").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_function_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFunctionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.data_source_name {
        object.key("dataSourceName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.description {
        object.key("description").string(var_25.as_str());
    }
    if let Some(var_26) = &input.function_version {
        object.key("functionVersion").string(var_26.as_str());
    }
    if input.max_batch_size != 0 {
        object.key("maxBatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_batch_size).into()),
        );
    }
    if let Some(var_27) = &input.name {
        object.key("name").string(var_27.as_str());
    }
    if let Some(var_28) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_28.as_str());
    }
    if let Some(var_29) = &input.response_mapping_template {
        object
            .key("responseMappingTemplate")
            .string(var_29.as_str());
    }
    if let Some(var_30) = &input.sync_config {
        #[allow(unused_mut)]
        let mut object_31 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_sync_config(&mut object_31, var_30)?;
        object_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_graphql_api_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGraphqlApiInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.additional_authentication_providers {
        let mut array_33 = object
            .key("additionalAuthenticationProviders")
            .start_array();
        for item_34 in var_32 {
            {
                #[allow(unused_mut)]
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_crate_model_additional_authentication_provider(&mut object_35, item_34)?;
                object_35.finish();
            }
        }
        array_33.finish();
    }
    if let Some(var_36) = &input.authentication_type {
        object.key("authenticationType").string(var_36.as_str());
    }
    if let Some(var_37) = &input.lambda_authorizer_config {
        #[allow(unused_mut)]
        let mut object_38 = object.key("lambdaAuthorizerConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_authorizer_config(
            &mut object_38,
            var_37,
        )?;
        object_38.finish();
    }
    if let Some(var_39) = &input.log_config {
        #[allow(unused_mut)]
        let mut object_40 = object.key("logConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_log_config(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.name {
        object.key("name").string(var_41.as_str());
    }
    if let Some(var_42) = &input.open_id_connect_config {
        #[allow(unused_mut)]
        let mut object_43 = object.key("openIDConnectConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_open_id_connect_config(
            &mut object_43,
            var_42,
        )?;
        object_43.finish();
    }
    if let Some(var_44) = &input.tags {
        #[allow(unused_mut)]
        let mut object_45 = object.key("tags").start_object();
        for (key_46, value_47) in var_44 {
            {
                object_45.key(key_46.as_str()).string(value_47.as_str());
            }
        }
        object_45.finish();
    }
    if let Some(var_48) = &input.user_pool_config {
        #[allow(unused_mut)]
        let mut object_49 = object.key("userPoolConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_user_pool_config(&mut object_49, var_48)?;
        object_49.finish();
    }
    if input.xray_enabled {
        object.key("xrayEnabled").boolean(input.xray_enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_resolver_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResolverInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.caching_config {
        #[allow(unused_mut)]
        let mut object_51 = object.key("cachingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_caching_config(&mut object_51, var_50)?;
        object_51.finish();
    }
    if let Some(var_52) = &input.data_source_name {
        object.key("dataSourceName").string(var_52.as_str());
    }
    if let Some(var_53) = &input.field_name {
        object.key("fieldName").string(var_53.as_str());
    }
    if let Some(var_54) = &input.kind {
        object.key("kind").string(var_54.as_str());
    }
    if input.max_batch_size != 0 {
        object.key("maxBatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_batch_size).into()),
        );
    }
    if let Some(var_55) = &input.pipeline_config {
        #[allow(unused_mut)]
        let mut object_56 = object.key("pipelineConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_pipeline_config(&mut object_56, var_55)?;
        object_56.finish();
    }
    if let Some(var_57) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_57.as_str());
    }
    if let Some(var_58) = &input.response_mapping_template {
        object
            .key("responseMappingTemplate")
            .string(var_58.as_str());
    }
    if let Some(var_59) = &input.sync_config {
        #[allow(unused_mut)]
        let mut object_60 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_sync_config(&mut object_60, var_59)?;
        object_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_type_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTypeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.definition {
        object.key("definition").string(var_61.as_str());
    }
    if let Some(var_62) = &input.format {
        object.key("format").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_evaluate_mapping_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EvaluateMappingTemplateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.context {
        object.key("context").string(var_63.as_str());
    }
    if let Some(var_64) = &input.template {
        object.key("template").string(var_64.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_schema_creation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartSchemaCreationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.definition {
        object
            .key("definition")
            .string_unchecked(&aws_smithy_types::base64::encode(var_65));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.tags {
        #[allow(unused_mut)]
        let mut object_67 = object.key("tags").start_object();
        for (key_68, value_69) in var_66 {
            {
                object_67.key(key_68.as_str()).string(value_69.as_str());
            }
        }
        object_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_api_cache_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApiCacheInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.api_caching_behavior {
        object.key("apiCachingBehavior").string(var_70.as_str());
    }
    {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    if let Some(var_71) = &input.r#type {
        object.key("type").string(var_71.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_api_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApiKeyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.description {
        object.key("description").string(var_72.as_str());
    }
    if input.expires != 0 {
        object.key("expires").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.expires).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_data_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataSourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.description {
        object.key("description").string(var_73.as_str());
    }
    if let Some(var_74) = &input.dynamodb_config {
        #[allow(unused_mut)]
        let mut object_75 = object.key("dynamodbConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dynamodb_data_source_config(
            &mut object_75,
            var_74,
        )?;
        object_75.finish();
    }
    if let Some(var_76) = &input.elasticsearch_config {
        #[allow(unused_mut)]
        let mut object_77 = object.key("elasticsearchConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_elasticsearch_data_source_config(
            &mut object_77,
            var_76,
        )?;
        object_77.finish();
    }
    if let Some(var_78) = &input.http_config {
        #[allow(unused_mut)]
        let mut object_79 = object.key("httpConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_http_data_source_config(
            &mut object_79,
            var_78,
        )?;
        object_79.finish();
    }
    if let Some(var_80) = &input.lambda_config {
        #[allow(unused_mut)]
        let mut object_81 = object.key("lambdaConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_data_source_config(
            &mut object_81,
            var_80,
        )?;
        object_81.finish();
    }
    if let Some(var_82) = &input.open_search_service_config {
        #[allow(unused_mut)]
        let mut object_83 = object.key("openSearchServiceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_open_search_service_data_source_config(
            &mut object_83,
            var_82,
        )?;
        object_83.finish();
    }
    if let Some(var_84) = &input.relational_database_config {
        #[allow(unused_mut)]
        let mut object_85 = object.key("relationalDatabaseConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_relational_database_data_source_config(
            &mut object_85,
            var_84,
        )?;
        object_85.finish();
    }
    if let Some(var_86) = &input.service_role_arn {
        object.key("serviceRoleArn").string(var_86.as_str());
    }
    if let Some(var_87) = &input.r#type {
        object.key("type").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_name_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainNameInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.description {
        object.key("description").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_function_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.data_source_name {
        object.key("dataSourceName").string(var_89.as_str());
    }
    if let Some(var_90) = &input.description {
        object.key("description").string(var_90.as_str());
    }
    if let Some(var_91) = &input.function_version {
        object.key("functionVersion").string(var_91.as_str());
    }
    if input.max_batch_size != 0 {
        object.key("maxBatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_batch_size).into()),
        );
    }
    if let Some(var_92) = &input.name {
        object.key("name").string(var_92.as_str());
    }
    if let Some(var_93) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_93.as_str());
    }
    if let Some(var_94) = &input.response_mapping_template {
        object
            .key("responseMappingTemplate")
            .string(var_94.as_str());
    }
    if let Some(var_95) = &input.sync_config {
        #[allow(unused_mut)]
        let mut object_96 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_sync_config(&mut object_96, var_95)?;
        object_96.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_graphql_api_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGraphqlApiInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.additional_authentication_providers {
        let mut array_98 = object
            .key("additionalAuthenticationProviders")
            .start_array();
        for item_99 in var_97 {
            {
                #[allow(unused_mut)]
                let mut object_100 = array_98.value().start_object();
                crate::json_ser::serialize_structure_crate_model_additional_authentication_provider(&mut object_100, item_99)?;
                object_100.finish();
            }
        }
        array_98.finish();
    }
    if let Some(var_101) = &input.authentication_type {
        object.key("authenticationType").string(var_101.as_str());
    }
    if let Some(var_102) = &input.lambda_authorizer_config {
        #[allow(unused_mut)]
        let mut object_103 = object.key("lambdaAuthorizerConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_authorizer_config(
            &mut object_103,
            var_102,
        )?;
        object_103.finish();
    }
    if let Some(var_104) = &input.log_config {
        #[allow(unused_mut)]
        let mut object_105 = object.key("logConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_log_config(&mut object_105, var_104)?;
        object_105.finish();
    }
    if let Some(var_106) = &input.name {
        object.key("name").string(var_106.as_str());
    }
    if let Some(var_107) = &input.open_id_connect_config {
        #[allow(unused_mut)]
        let mut object_108 = object.key("openIDConnectConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_open_id_connect_config(
            &mut object_108,
            var_107,
        )?;
        object_108.finish();
    }
    if let Some(var_109) = &input.user_pool_config {
        #[allow(unused_mut)]
        let mut object_110 = object.key("userPoolConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_user_pool_config(
            &mut object_110,
            var_109,
        )?;
        object_110.finish();
    }
    if input.xray_enabled {
        object.key("xrayEnabled").boolean(input.xray_enabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_resolver_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResolverInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_111) = &input.caching_config {
        #[allow(unused_mut)]
        let mut object_112 = object.key("cachingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_caching_config(&mut object_112, var_111)?;
        object_112.finish();
    }
    if let Some(var_113) = &input.data_source_name {
        object.key("dataSourceName").string(var_113.as_str());
    }
    if let Some(var_114) = &input.kind {
        object.key("kind").string(var_114.as_str());
    }
    if input.max_batch_size != 0 {
        object.key("maxBatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_batch_size).into()),
        );
    }
    if let Some(var_115) = &input.pipeline_config {
        #[allow(unused_mut)]
        let mut object_116 = object.key("pipelineConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_pipeline_config(&mut object_116, var_115)?;
        object_116.finish();
    }
    if let Some(var_117) = &input.request_mapping_template {
        object
            .key("requestMappingTemplate")
            .string(var_117.as_str());
    }
    if let Some(var_118) = &input.response_mapping_template {
        object
            .key("responseMappingTemplate")
            .string(var_118.as_str());
    }
    if let Some(var_119) = &input.sync_config {
        #[allow(unused_mut)]
        let mut object_120 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_sync_config(&mut object_120, var_119)?;
        object_120.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_type_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTypeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.definition {
        object.key("definition").string(var_121.as_str());
    }
    if let Some(var_122) = &input.format {
        object.key("format").string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dynamodb_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DynamodbDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_123) = &input.table_name {
        object.key("tableName").string(var_123.as_str());
    }
    if let Some(var_124) = &input.aws_region {
        object.key("awsRegion").string(var_124.as_str());
    }
    if input.use_caller_credentials {
        object
            .key("useCallerCredentials")
            .boolean(input.use_caller_credentials);
    }
    if let Some(var_125) = &input.delta_sync_config {
        #[allow(unused_mut)]
        let mut object_126 = object.key("deltaSyncConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_delta_sync_config(
            &mut object_126,
            var_125,
        )?;
        object_126.finish();
    }
    if input.versioned {
        object.key("versioned").boolean(input.versioned);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_elasticsearch_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ElasticsearchDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.endpoint {
        object.key("endpoint").string(var_127.as_str());
    }
    if let Some(var_128) = &input.aws_region {
        object.key("awsRegion").string(var_128.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_http_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.endpoint {
        object.key("endpoint").string(var_129.as_str());
    }
    if let Some(var_130) = &input.authorization_config {
        #[allow(unused_mut)]
        let mut object_131 = object.key("authorizationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_authorization_config(
            &mut object_131,
            var_130,
        )?;
        object_131.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_132) = &input.lambda_function_arn {
        object.key("lambdaFunctionArn").string(var_132.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_open_search_service_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OpenSearchServiceDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_133) = &input.endpoint {
        object.key("endpoint").string(var_133.as_str());
    }
    if let Some(var_134) = &input.aws_region {
        object.key("awsRegion").string(var_134.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_relational_database_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelationalDatabaseDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_135) = &input.relational_database_source_type {
        object
            .key("relationalDatabaseSourceType")
            .string(var_135.as_str());
    }
    if let Some(var_136) = &input.rds_http_endpoint_config {
        #[allow(unused_mut)]
        let mut object_137 = object.key("rdsHttpEndpointConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_rds_http_endpoint_config(
            &mut object_137,
            var_136,
        )?;
        object_137.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sync_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SyncConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_138) = &input.conflict_handler {
        object.key("conflictHandler").string(var_138.as_str());
    }
    if let Some(var_139) = &input.conflict_detection {
        object.key("conflictDetection").string(var_139.as_str());
    }
    if let Some(var_140) = &input.lambda_conflict_handler_config {
        #[allow(unused_mut)]
        let mut object_141 = object.key("lambdaConflictHandlerConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_conflict_handler_config(
            &mut object_141,
            var_140,
        )?;
        object_141.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_additional_authentication_provider(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdditionalAuthenticationProvider,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_142) = &input.authentication_type {
        object.key("authenticationType").string(var_142.as_str());
    }
    if let Some(var_143) = &input.open_id_connect_config {
        #[allow(unused_mut)]
        let mut object_144 = object.key("openIDConnectConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_open_id_connect_config(
            &mut object_144,
            var_143,
        )?;
        object_144.finish();
    }
    if let Some(var_145) = &input.user_pool_config {
        #[allow(unused_mut)]
        let mut object_146 = object.key("userPoolConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_cognito_user_pool_config(
            &mut object_146,
            var_145,
        )?;
        object_146.finish();
    }
    if let Some(var_147) = &input.lambda_authorizer_config {
        #[allow(unused_mut)]
        let mut object_148 = object.key("lambdaAuthorizerConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_authorizer_config(
            &mut object_148,
            var_147,
        )?;
        object_148.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_authorizer_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaAuthorizerConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.authorizer_result_ttl_in_seconds != 0 {
        object.key("authorizerResultTtlInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.authorizer_result_ttl_in_seconds).into()),
        );
    }
    if let Some(var_149) = &input.authorizer_uri {
        object.key("authorizerUri").string(var_149.as_str());
    }
    if let Some(var_150) = &input.identity_validation_expression {
        object
            .key("identityValidationExpression")
            .string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_151) = &input.field_log_level {
        object.key("fieldLogLevel").string(var_151.as_str());
    }
    if let Some(var_152) = &input.cloud_watch_logs_role_arn {
        object.key("cloudWatchLogsRoleArn").string(var_152.as_str());
    }
    if input.exclude_verbose_content {
        object
            .key("excludeVerboseContent")
            .boolean(input.exclude_verbose_content);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_open_id_connect_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OpenIdConnectConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_153) = &input.issuer {
        object.key("issuer").string(var_153.as_str());
    }
    if let Some(var_154) = &input.client_id {
        object.key("clientId").string(var_154.as_str());
    }
    if input.iat_ttl != 0 {
        object.key("iatTTL").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.iat_ttl).into()),
        );
    }
    if input.auth_ttl != 0 {
        object.key("authTTL").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.auth_ttl).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_user_pool_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UserPoolConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_155) = &input.user_pool_id {
        object.key("userPoolId").string(var_155.as_str());
    }
    if let Some(var_156) = &input.aws_region {
        object.key("awsRegion").string(var_156.as_str());
    }
    if let Some(var_157) = &input.default_action {
        object.key("defaultAction").string(var_157.as_str());
    }
    if let Some(var_158) = &input.app_id_client_regex {
        object.key("appIdClientRegex").string(var_158.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_caching_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CachingConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    if let Some(var_159) = &input.caching_keys {
        let mut array_160 = object.key("cachingKeys").start_array();
        for item_161 in var_159 {
            {
                array_160.value().string(item_161.as_str());
            }
        }
        array_160.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_pipeline_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PipelineConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_162) = &input.functions {
        let mut array_163 = object.key("functions").start_array();
        for item_164 in var_162 {
            {
                array_163.value().string(item_164.as_str());
            }
        }
        array_163.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_delta_sync_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeltaSyncConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.base_table_ttl != 0 {
        object.key("baseTableTTL").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.base_table_ttl).into()),
        );
    }
    if let Some(var_165) = &input.delta_sync_table_name {
        object.key("deltaSyncTableName").string(var_165.as_str());
    }
    if input.delta_sync_table_ttl != 0 {
        object.key("deltaSyncTableTTL").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.delta_sync_table_ttl).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_authorization_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AuthorizationConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_166) = &input.authorization_type {
        object.key("authorizationType").string(var_166.as_str());
    }
    if let Some(var_167) = &input.aws_iam_config {
        #[allow(unused_mut)]
        let mut object_168 = object.key("awsIamConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_iam_config(&mut object_168, var_167)?;
        object_168.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rds_http_endpoint_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RdsHttpEndpointConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_169) = &input.aws_region {
        object.key("awsRegion").string(var_169.as_str());
    }
    if let Some(var_170) = &input.db_cluster_identifier {
        object.key("dbClusterIdentifier").string(var_170.as_str());
    }
    if let Some(var_171) = &input.database_name {
        object.key("databaseName").string(var_171.as_str());
    }
    if let Some(var_172) = &input.schema {
        object.key("schema").string(var_172.as_str());
    }
    if let Some(var_173) = &input.aws_secret_store_arn {
        object.key("awsSecretStoreArn").string(var_173.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_conflict_handler_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaConflictHandlerConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_174) = &input.lambda_conflict_handler_arn {
        object
            .key("lambdaConflictHandlerArn")
            .string(var_174.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cognito_user_pool_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CognitoUserPoolConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_175) = &input.user_pool_id {
        object.key("userPoolId").string(var_175.as_str());
    }
    if let Some(var_176) = &input.aws_region {
        object.key("awsRegion").string(var_176.as_str());
    }
    if let Some(var_177) = &input.app_id_client_regex {
        object.key("appIdClientRegex").string(var_177.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_aws_iam_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsIamConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_178) = &input.signing_region {
        object.key("signingRegion").string(var_178.as_str());
    }
    if let Some(var_179) = &input.signing_service_name {
        object.key("signingServiceName").string(var_179.as_str());
    }
    Ok(())
}
