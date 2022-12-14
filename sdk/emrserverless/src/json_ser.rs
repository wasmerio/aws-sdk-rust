// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.auto_start_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("autoStartConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_start_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.auto_stop_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("autoStopConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_stop_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.client_token {
        object.key("clientToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.initial_capacity {
        #[allow(unused_mut)]
        let mut object_7 = object.key("initialCapacity").start_object();
        for (key_8, value_9) in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_10 = object_7.key(key_8.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_initial_capacity_config(
                    &mut object_10,
                    value_9,
                )?;
                object_10.finish();
            }
        }
        object_7.finish();
    }
    if let Some(var_11) = &input.maximum_capacity {
        #[allow(unused_mut)]
        let mut object_12 = object.key("maximumCapacity").start_object();
        crate::json_ser::serialize_structure_crate_model_maximum_allowed_resources(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.name {
        object.key("name").string(var_13.as_str());
    }
    if let Some(var_14) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_15 = object.key("networkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_configuration(
            &mut object_15,
            var_14,
        )?;
        object_15.finish();
    }
    if let Some(var_16) = &input.release_label {
        object.key("releaseLabel").string(var_16.as_str());
    }
    if let Some(var_17) = &input.tags {
        #[allow(unused_mut)]
        let mut object_18 = object.key("tags").start_object();
        for (key_19, value_20) in var_17 {
            {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    if let Some(var_21) = &input.r#type {
        object.key("type").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_job_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartJobRunInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.client_token {
        object.key("clientToken").string(var_22.as_str());
    }
    if let Some(var_23) = &input.configuration_overrides {
        #[allow(unused_mut)]
        let mut object_24 = object.key("configurationOverrides").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_overrides(
            &mut object_24,
            var_23,
        )?;
        object_24.finish();
    }
    if let Some(var_25) = &input.execution_role_arn {
        object.key("executionRoleArn").string(var_25.as_str());
    }
    if let Some(var_26) = &input.execution_timeout_minutes {
        object.key("executionTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_26).into()),
        );
    }
    if let Some(var_27) = &input.job_driver {
        #[allow(unused_mut)]
        let mut object_28 = object.key("jobDriver").start_object();
        crate::json_ser::serialize_union_crate_model_job_driver(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.name {
        object.key("name").string(var_29.as_str());
    }
    if let Some(var_30) = &input.tags {
        #[allow(unused_mut)]
        let mut object_31 = object.key("tags").start_object();
        for (key_32, value_33) in var_30 {
            {
                object_31.key(key_32.as_str()).string(value_33.as_str());
            }
        }
        object_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.tags {
        #[allow(unused_mut)]
        let mut object_35 = object.key("tags").start_object();
        for (key_36, value_37) in var_34 {
            {
                object_35.key(key_36.as_str()).string(value_37.as_str());
            }
        }
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.auto_start_configuration {
        #[allow(unused_mut)]
        let mut object_39 = object.key("autoStartConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_start_config(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.auto_stop_configuration {
        #[allow(unused_mut)]
        let mut object_41 = object.key("autoStopConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_stop_config(&mut object_41, var_40)?;
        object_41.finish();
    }
    if let Some(var_42) = &input.client_token {
        object.key("clientToken").string(var_42.as_str());
    }
    if let Some(var_43) = &input.initial_capacity {
        #[allow(unused_mut)]
        let mut object_44 = object.key("initialCapacity").start_object();
        for (key_45, value_46) in var_43 {
            {
                #[allow(unused_mut)]
                let mut object_47 = object_44.key(key_45.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_initial_capacity_config(
                    &mut object_47,
                    value_46,
                )?;
                object_47.finish();
            }
        }
        object_44.finish();
    }
    if let Some(var_48) = &input.maximum_capacity {
        #[allow(unused_mut)]
        let mut object_49 = object.key("maximumCapacity").start_object();
        crate::json_ser::serialize_structure_crate_model_maximum_allowed_resources(
            &mut object_49,
            var_48,
        )?;
        object_49.finish();
    }
    if let Some(var_50) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_51 = object.key("networkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_configuration(
            &mut object_51,
            var_50,
        )?;
        object_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_start_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoStartConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.enabled {
        object.key("enabled").boolean(*var_52);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_stop_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoStopConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.enabled {
        object.key("enabled").boolean(*var_53);
    }
    if let Some(var_54) = &input.idle_timeout_minutes {
        object.key("idleTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_initial_capacity_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InitialCapacityConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("workerCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.worker_count).into()),
        );
    }
    if let Some(var_55) = &input.worker_configuration {
        #[allow(unused_mut)]
        let mut object_56 = object.key("workerConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_worker_resource_config(
            &mut object_56,
            var_55,
        )?;
        object_56.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_maximum_allowed_resources(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MaximumAllowedResources,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.cpu {
        object.key("cpu").string(var_57.as_str());
    }
    if let Some(var_58) = &input.memory {
        object.key("memory").string(var_58.as_str());
    }
    if let Some(var_59) = &input.disk {
        object.key("disk").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NetworkConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.subnet_ids {
        let mut array_61 = object.key("subnetIds").start_array();
        for item_62 in var_60 {
            {
                array_61.value().string(item_62.as_str());
            }
        }
        array_61.finish();
    }
    if let Some(var_63) = &input.security_group_ids {
        let mut array_64 = object.key("securityGroupIds").start_array();
        for item_65 in var_63 {
            {
                array_64.value().string(item_65.as_str());
            }
        }
        array_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_configuration_overrides(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConfigurationOverrides,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.application_configuration {
        let mut array_67 = object.key("applicationConfiguration").start_array();
        for item_68 in var_66 {
            {
                #[allow(unused_mut)]
                let mut object_69 = array_67.value().start_object();
                crate::json_ser::serialize_structure_crate_model_configuration(
                    &mut object_69,
                    item_68,
                )?;
                object_69.finish();
            }
        }
        array_67.finish();
    }
    if let Some(var_70) = &input.monitoring_configuration {
        #[allow(unused_mut)]
        let mut object_71 = object.key("monitoringConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_monitoring_configuration(
            &mut object_71,
            var_70,
        )?;
        object_71.finish();
    }
    Ok(())
}

pub fn serialize_union_crate_model_job_driver(
    object_28: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobDriver,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::JobDriver::SparkSubmit(inner) => {
            #[allow(unused_mut)]
            let mut object_72 = object_28.key("sparkSubmit").start_object();
            crate::json_ser::serialize_structure_crate_model_spark_submit(&mut object_72, inner)?;
            object_72.finish();
        }
        crate::model::JobDriver::Hive(inner) => {
            #[allow(unused_mut)]
            let mut object_73 = object_28.key("hive").start_object();
            crate::json_ser::serialize_structure_crate_model_hive(&mut object_73, inner)?;
            object_73.finish();
        }
        crate::model::JobDriver::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant("JobDriver"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_worker_resource_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WorkerResourceConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.cpu {
        object.key("cpu").string(var_74.as_str());
    }
    if let Some(var_75) = &input.memory {
        object.key("memory").string(var_75.as_str());
    }
    if let Some(var_76) = &input.disk {
        object.key("disk").string(var_76.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Configuration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_77) = &input.classification {
        object.key("classification").string(var_77.as_str());
    }
    if let Some(var_78) = &input.properties {
        #[allow(unused_mut)]
        let mut object_79 = object.key("properties").start_object();
        for (key_80, value_81) in var_78 {
            {
                object_79.key(key_80.as_str()).string(value_81.as_str());
            }
        }
        object_79.finish();
    }
    if let Some(var_82) = &input.configurations {
        let mut array_83 = object.key("configurations").start_array();
        for item_84 in var_82 {
            {
                #[allow(unused_mut)]
                let mut object_85 = array_83.value().start_object();
                crate::json_ser::serialize_structure_crate_model_configuration(
                    &mut object_85,
                    item_84,
                )?;
                object_85.finish();
            }
        }
        array_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_monitoring_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MonitoringConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.s3_monitoring_configuration {
        #[allow(unused_mut)]
        let mut object_87 = object.key("s3MonitoringConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_monitoring_configuration(
            &mut object_87,
            var_86,
        )?;
        object_87.finish();
    }
    if let Some(var_88) = &input.managed_persistence_monitoring_configuration {
        #[allow(unused_mut)]
        let mut object_89 = object
            .key("managedPersistenceMonitoringConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_managed_persistence_monitoring_configuration(&mut object_89, var_88)?;
        object_89.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_spark_submit(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SparkSubmit,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.entry_point {
        object.key("entryPoint").string(var_90.as_str());
    }
    if let Some(var_91) = &input.entry_point_arguments {
        let mut array_92 = object.key("entryPointArguments").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
    if let Some(var_94) = &input.spark_submit_parameters {
        object.key("sparkSubmitParameters").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hive(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Hive,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.query {
        object.key("query").string(var_95.as_str());
    }
    if let Some(var_96) = &input.init_query_file {
        object.key("initQueryFile").string(var_96.as_str());
    }
    if let Some(var_97) = &input.parameters {
        object.key("parameters").string(var_97.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_monitoring_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3MonitoringConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_98) = &input.log_uri {
        object.key("logUri").string(var_98.as_str());
    }
    if let Some(var_99) = &input.encryption_key_arn {
        object.key("encryptionKeyArn").string(var_99.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_managed_persistence_monitoring_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManagedPersistenceMonitoringConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.enabled {
        object.key("enabled").boolean(*var_100);
    }
    if let Some(var_101) = &input.encryption_key_arn {
        object.key("encryptionKeyArn").string(var_101.as_str());
    }
    Ok(())
}
