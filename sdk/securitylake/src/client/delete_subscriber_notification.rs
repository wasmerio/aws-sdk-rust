// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSubscriberNotification`](crate::operation::delete_subscriber_notification::builders::DeleteSubscriberNotificationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`subscriber_id(impl ::std::convert::Into<String>)`](crate::operation::delete_subscriber_notification::builders::DeleteSubscriberNotificationFluentBuilder::subscriber_id) / [`set_subscriber_id(Option<String>)`](crate::operation::delete_subscriber_notification::builders::DeleteSubscriberNotificationFluentBuilder::set_subscriber_id): <p>The ID of the Security Lake subscriber account.</p>
    /// - On success, responds with [`DeleteSubscriberNotificationOutput`](crate::operation::delete_subscriber_notification::DeleteSubscriberNotificationOutput)
    /// - On failure, responds with [`SdkError<DeleteSubscriberNotificationError>`](crate::operation::delete_subscriber_notification::DeleteSubscriberNotificationError)
    pub fn delete_subscriber_notification(
        &self,
    ) -> crate::operation::delete_subscriber_notification::builders::DeleteSubscriberNotificationFluentBuilder {
        crate::operation::delete_subscriber_notification::builders::DeleteSubscriberNotificationFluentBuilder::new(self.handle.clone())
    }
}
