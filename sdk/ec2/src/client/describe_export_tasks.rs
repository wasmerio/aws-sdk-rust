// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeExportTasks`](crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`export_task_ids(Vec<String>)`](crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder::export_task_ids) / [`set_export_task_ids(Option<Vec<String>>)`](crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder::set_export_task_ids): <p>The export task IDs.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder::set_filters): <p>the filters for the export tasks.</p>
    /// - On success, responds with [`DescribeExportTasksOutput`](crate::operation::describe_export_tasks::DescribeExportTasksOutput) with field(s):
    ///   - [`export_tasks(Option<Vec<ExportTask>>)`](crate::operation::describe_export_tasks::DescribeExportTasksOutput::export_tasks): <p>Information about the export tasks.</p>
    /// - On failure, responds with [`SdkError<DescribeExportTasksError>`](crate::operation::describe_export_tasks::DescribeExportTasksError)
    pub fn describe_export_tasks(&self) -> crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder {
        crate::operation::describe_export_tasks::builders::DescribeExportTasksFluentBuilder::new(self.handle.clone())
    }
}
