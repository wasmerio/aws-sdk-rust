// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteQueue`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`queue_id(impl ::std::convert::Into<String>)`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::queue_id) / [`set_queue_id(Option<String>)`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::set_queue_id): <p>The identifier for the queue.</p>
    /// - On success, responds with [`DeleteQueueOutput`](crate::operation::delete_queue::DeleteQueueOutput)
    /// - On failure, responds with [`SdkError<DeleteQueueError>`](crate::operation::delete_queue::DeleteQueueError)
    pub fn delete_queue(&self) -> crate::operation::delete_queue::builders::DeleteQueueFluentBuilder {
        crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::new(self.handle.clone())
    }
}
