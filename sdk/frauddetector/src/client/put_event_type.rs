// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutEventType`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_name): <p>The name.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_description): <p>The description of the event type.</p>
    ///   - [`event_variables(Vec<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::event_variables) / [`set_event_variables(Option<Vec<String>>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_event_variables): <p>The event type variables.</p>
    ///   - [`labels(Vec<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::labels) / [`set_labels(Option<Vec<String>>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_labels): <p>The event type labels.</p>
    ///   - [`entity_types(Vec<String>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::entity_types) / [`set_entity_types(Option<Vec<String>>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_entity_types): <p>The entity type for the event type. Example entity types: customer, merchant, account.</p>
    ///   - [`event_ingestion(EventIngestion)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::event_ingestion) / [`set_event_ingestion(Option<EventIngestion>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_event_ingestion): <p>Specifies if ingestion is enabled or disabled.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_tags): <p>A collection of key and value pairs.</p>
    ///   - [`event_orchestration(EventOrchestration)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::event_orchestration) / [`set_event_orchestration(Option<EventOrchestration>)`](crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::set_event_orchestration): <p>Enables or disables event orchestration. If enabled, you can send event predictions to select AWS services for downstream processing of the events.</p>
    /// - On success, responds with [`PutEventTypeOutput`](crate::operation::put_event_type::PutEventTypeOutput)
    /// - On failure, responds with [`SdkError<PutEventTypeError>`](crate::operation::put_event_type::PutEventTypeError)
    pub fn put_event_type(&self) -> crate::operation::put_event_type::builders::PutEventTypeFluentBuilder {
        crate::operation::put_event_type::builders::PutEventTypeFluentBuilder::new(self.handle.clone())
    }
}
