// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchUpdateSchedule`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_id(impl ::std::convert::Into<String>)`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::channel_id) / [`set_channel_id(Option<String>)`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::set_channel_id): Id of the channel whose schedule is being updated.
    ///   - [`creates(BatchScheduleActionCreateRequest)`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::creates) / [`set_creates(Option<BatchScheduleActionCreateRequest>)`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::set_creates): Schedule actions to create in the schedule.
    ///   - [`deletes(BatchScheduleActionDeleteRequest)`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::deletes) / [`set_deletes(Option<BatchScheduleActionDeleteRequest>)`](crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::set_deletes): Schedule actions to delete from the schedule.
    /// - On success, responds with [`BatchUpdateScheduleOutput`](crate::operation::batch_update_schedule::BatchUpdateScheduleOutput) with field(s):
    ///   - [`creates(Option<BatchScheduleActionCreateResult>)`](crate::operation::batch_update_schedule::BatchUpdateScheduleOutput::creates): Schedule actions created in the schedule.
    ///   - [`deletes(Option<BatchScheduleActionDeleteResult>)`](crate::operation::batch_update_schedule::BatchUpdateScheduleOutput::deletes): Schedule actions deleted from the schedule.
    /// - On failure, responds with [`SdkError<BatchUpdateScheduleError>`](crate::operation::batch_update_schedule::BatchUpdateScheduleError)
    pub fn batch_update_schedule(&self) -> crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder {
        crate::operation::batch_update_schedule::builders::BatchUpdateScheduleFluentBuilder::new(self.handle.clone())
    }
}
