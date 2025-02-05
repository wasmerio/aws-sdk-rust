// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl ListSchemaVersionsInput {}
/// Orchestration and serialization glue logic for `ListSchemaVersions`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ListSchemaVersions;
impl ListSchemaVersions {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::list_schema_versions::ListSchemaVersionsInput,
    ) -> ::std::result::Result<
        crate::operation::list_schema_versions::ListSchemaVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_schema_versions::ListSchemaVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::list_schema_versions::ListSchemaVersionsError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::list_schema_versions::ListSchemaVersionsOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::list_schema_versions::ListSchemaVersionsInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("schemas", "ListSchemaVersions", input, runtime_plugins, stop_point).await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());

        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListSchemaVersions {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListSchemaVersions");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            ListSchemaVersionsRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            ListSchemaVersionsResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new("ListSchemaVersions", "schemas"));
        let mut signing_options = ::aws_runtime::auth::sigv4::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::sigv4::SigV4OperationSigningConfig {
            region: None,
            service: None,
            signing_options,
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(&self) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        // Retry classifiers are operation-specific because they need to downcast operation-specific error types.
        let retry_classifiers = ::aws_smithy_runtime_api::client::retries::RetryClassifiers::new()
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::SmithyErrorClassifier::<
                crate::operation::list_schema_versions::ListSchemaVersionsError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AmzRetryAfterHeaderClassifier)
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::ModeledAsRetryableClassifier::<
                crate::operation::list_schema_versions::ListSchemaVersionsError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AwsErrorCodeClassifier::<
                crate::operation::list_schema_versions::ListSchemaVersionsError,
            >::new())
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::HttpStatusCodeClassifier::default());

        ::std::borrow::Cow::Owned(
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListSchemaVersions")
                .with_retry_classifiers(::std::option::Option::Some(retry_classifiers))
                .with_auth_scheme_option_resolver(::std::option::Option::Some(
                    ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(
                        ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver::new(vec![
                            ::aws_runtime::auth::sigv4::SCHEME_ID,
                        ]),
                    ),
                ))
                .with_interceptor(
                    ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(ListSchemaVersionsEndpointParamsInterceptor) as _,
                ),
        )
    }
}

#[derive(Debug)]
struct ListSchemaVersionsResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::ResponseDeserializer for ListSchemaVersionsResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 {
            crate::protocol_serde::shape_list_schema_versions::de_list_schema_versions_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_list_schema_versions::de_list_schema_versions_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct ListSchemaVersionsRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::RequestSerializer for ListSchemaVersionsRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::list_schema_versions::ListSchemaVersionsInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::list_schema_versions::ListSchemaVersionsInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                use ::std::fmt::Write as _;
                let input_1 = &_input.registry_name;
                let input_1 = input_1
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_http::operation::error::BuildError::missing_field("registry_name", "cannot be empty or unset"))?;
                let registry_name = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                if registry_name.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_http::operation::error::BuildError::missing_field(
                        "registry_name",
                        "cannot be empty or unset",
                    ));
                }
                let input_2 = &_input.schema_name;
                let input_2 = input_2
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_http::operation::error::BuildError::missing_field("schema_name", "cannot be empty or unset"))?;
                let schema_name = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Default);
                if schema_name.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_http::operation::error::BuildError::missing_field(
                        "schema_name",
                        "cannot be empty or unset",
                    ));
                }
                ::std::write!(
                    output,
                    "/v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/versions",
                    RegistryName = registry_name,
                    SchemaName = schema_name
                )
                .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::list_schema_versions::ListSchemaVersionsInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                if let ::std::option::Option::Some(inner_3) = &_input.limit {
                    if *inner_3 != 0 {
                        query.push_kv("limit", ::aws_smithy_types::primitive::Encoder::from(*inner_3).encode());
                    }
                }
                if let ::std::option::Option::Some(inner_4) = &_input.next_token {
                    {
                        query.push_kv("nextToken", &::aws_smithy_http::query::fmt_string(&inner_4));
                    }
                }
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::list_schema_versions::ListSchemaVersionsInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder
        };
        let body = ::aws_smithy_http::body::SdkBody::from("");

        ::std::result::Result::Ok(request_builder.body(body).expect("valid request"))
    }
}
#[derive(Debug)]
struct ListSchemaVersionsEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Interceptor for ListSchemaVersionsEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "ListSchemaVersionsEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<ListSchemaVersionsInput>()
            .ok_or("failed to downcast to ListSchemaVersionsInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ListSchemaVersionsErrorKind = ListSchemaVersionsError;
/// Error type for the `ListSchemaVersionsError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum ListSchemaVersionsError {
    #[allow(missing_docs)] // documentation missing in model
    BadRequestException(crate::types::error::BadRequestException),
    #[allow(missing_docs)] // documentation missing in model
    ForbiddenException(crate::types::error::ForbiddenException),
    #[allow(missing_docs)] // documentation missing in model
    InternalServerErrorException(crate::types::error::InternalServerErrorException),
    #[allow(missing_docs)] // documentation missing in model
    NotFoundException(crate::types::error::NotFoundException),
    #[allow(missing_docs)] // documentation missing in model
    ServiceUnavailableException(crate::types::error::ServiceUnavailableException),
    #[allow(missing_docs)] // documentation missing in model
    UnauthorizedException(crate::types::error::UnauthorizedException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for ListSchemaVersionsError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for ListSchemaVersionsError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::BadRequestException(_inner) => _inner.fmt(f),
            Self::ForbiddenException(_inner) => _inner.fmt(f),
            Self::InternalServerErrorException(_inner) => _inner.fmt(f),
            Self::NotFoundException(_inner) => _inner.fmt(f),
            Self::ServiceUnavailableException(_inner) => _inner.fmt(f),
            Self::UnauthorizedException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ListSchemaVersionsError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::BadRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ForbiddenException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InternalServerErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ServiceUnavailableException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::list_schema_versions::ListSchemaVersionsError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for ListSchemaVersionsError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ListSchemaVersionsError {
    /// Creates the `ListSchemaVersionsError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err).build())
    }

    /// Creates the `ListSchemaVersionsError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::BadRequestException(e) => e.meta(),
            Self::ForbiddenException(e) => e.meta(),
            Self::InternalServerErrorException(e) => e.meta(),
            Self::NotFoundException(e) => e.meta(),
            Self::ServiceUnavailableException(e) => e.meta(),
            Self::UnauthorizedException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ListSchemaVersionsError::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(self, Self::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `ListSchemaVersionsError::ForbiddenException`.
    pub fn is_forbidden_exception(&self) -> bool {
        matches!(self, Self::ForbiddenException(_))
    }
    /// Returns `true` if the error kind is `ListSchemaVersionsError::InternalServerErrorException`.
    pub fn is_internal_server_error_exception(&self) -> bool {
        matches!(self, Self::InternalServerErrorException(_))
    }
    /// Returns `true` if the error kind is `ListSchemaVersionsError::NotFoundException`.
    pub fn is_not_found_exception(&self) -> bool {
        matches!(self, Self::NotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListSchemaVersionsError::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(self, Self::ServiceUnavailableException(_))
    }
    /// Returns `true` if the error kind is `ListSchemaVersionsError::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(self, Self::UnauthorizedException(_))
    }
}
impl ::std::error::Error for ListSchemaVersionsError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::BadRequestException(_inner) => ::std::option::Option::Some(_inner),
            Self::ForbiddenException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerErrorException(_inner) => ::std::option::Option::Some(_inner),
            Self::NotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceUnavailableException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnauthorizedException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::list_schema_versions::_list_schema_versions_output::ListSchemaVersionsOutput;

pub use crate::operation::list_schema_versions::_list_schema_versions_input::ListSchemaVersionsInput;

mod _list_schema_versions_input;

mod _list_schema_versions_output;

/// Builders
pub mod builders;

/// Paginator for this operation
pub mod paginator;
