// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAppsList`](crate::operation::put_apps_list::builders::PutAppsListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`apps_list(AppsListData)`](crate::operation::put_apps_list::builders::PutAppsListFluentBuilder::apps_list) / [`set_apps_list(Option<AppsListData>)`](crate::operation::put_apps_list::builders::PutAppsListFluentBuilder::set_apps_list): <p>The details of the Firewall Manager applications list to be created.</p>
    ///   - [`tag_list(Vec<Tag>)`](crate::operation::put_apps_list::builders::PutAppsListFluentBuilder::tag_list) / [`set_tag_list(Option<Vec<Tag>>)`](crate::operation::put_apps_list::builders::PutAppsListFluentBuilder::set_tag_list): <p>The tags associated with the resource.</p>
    /// - On success, responds with [`PutAppsListOutput`](crate::operation::put_apps_list::PutAppsListOutput) with field(s):
    ///   - [`apps_list(Option<AppsListData>)`](crate::operation::put_apps_list::PutAppsListOutput::apps_list): <p>The details of the Firewall Manager applications list.</p>
    ///   - [`apps_list_arn(Option<String>)`](crate::operation::put_apps_list::PutAppsListOutput::apps_list_arn): <p>The Amazon Resource Name (ARN) of the applications list.</p>
    /// - On failure, responds with [`SdkError<PutAppsListError>`](crate::operation::put_apps_list::PutAppsListError)
    pub fn put_apps_list(&self) -> crate::operation::put_apps_list::builders::PutAppsListFluentBuilder {
        crate::operation::put_apps_list::builders::PutAppsListFluentBuilder::new(self.handle.clone())
    }
}
