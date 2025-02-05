// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_invoke_async_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::invoke_async::InvokeAsyncOutput, crate::operation::invoke_async::InvokeAsyncError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::invoke_async::InvokeAsyncError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::invoke_async::InvokeAsyncError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::invoke_async::InvokeAsyncError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::invoke_async::InvokeAsyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRuntimeException" => crate::operation::invoke_async::InvokeAsyncError::InvalidRuntimeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRuntimeExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_runtime_exception::de_invalid_runtime_exception_json_err(_response_body, output)
                    .map_err(crate::operation::invoke_async::InvokeAsyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceException" => crate::operation::invoke_async::InvokeAsyncError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::invoke_async::InvokeAsyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRequestContentException" => crate::operation::invoke_async::InvokeAsyncError::InvalidRequestContentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestContentExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_content_exception::de_invalid_request_content_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::invoke_async::InvokeAsyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceConflictException" => crate::operation::invoke_async::InvokeAsyncError::ResourceConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::invoke_async::InvokeAsyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::invoke_async::InvokeAsyncError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_invoke_async_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::invoke_async::InvokeAsyncOutput, crate::operation::invoke_async::InvokeAsyncError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::invoke_async::builders::InvokeAsyncOutputBuilder::default();
        output = output.set_status(Some(_response_status as _));
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
