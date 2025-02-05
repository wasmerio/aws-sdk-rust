// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AcceptInputDeviceTransfer`](crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`input_device_id(impl ::std::convert::Into<String>)`](crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferFluentBuilder::input_device_id) / [`set_input_device_id(Option<String>)`](crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferFluentBuilder::set_input_device_id): The unique ID of the input device to accept. For example, hd-123456789abcdef.
    /// - On success, responds with [`AcceptInputDeviceTransferOutput`](crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferOutput)
    /// - On failure, responds with [`SdkError<AcceptInputDeviceTransferError>`](crate::operation::accept_input_device_transfer::AcceptInputDeviceTransferError)
    pub fn accept_input_device_transfer(&self) -> crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferFluentBuilder {
        crate::operation::accept_input_device_transfer::builders::AcceptInputDeviceTransferFluentBuilder::new(self.handle.clone())
    }
}
