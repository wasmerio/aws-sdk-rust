// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAnomalySubscriptionOutput {
    /// <p>A cost anomaly subscription ARN. </p>
    pub subscription_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateAnomalySubscriptionOutput {
    /// <p>A cost anomaly subscription ARN. </p>
    pub fn subscription_arn(&self) -> ::std::option::Option<&str> {
        self.subscription_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateAnomalySubscriptionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateAnomalySubscriptionOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAnomalySubscriptionOutput`](crate::operation::update_anomaly_subscription::UpdateAnomalySubscriptionOutput).
    pub fn builder() -> crate::operation::update_anomaly_subscription::builders::UpdateAnomalySubscriptionOutputBuilder {
        crate::operation::update_anomaly_subscription::builders::UpdateAnomalySubscriptionOutputBuilder::default()
    }
}

/// A builder for [`UpdateAnomalySubscriptionOutput`](crate::operation::update_anomaly_subscription::UpdateAnomalySubscriptionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateAnomalySubscriptionOutputBuilder {
    pub(crate) subscription_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateAnomalySubscriptionOutputBuilder {
    /// <p>A cost anomaly subscription ARN. </p>
    pub fn subscription_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subscription_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A cost anomaly subscription ARN. </p>
    pub fn set_subscription_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subscription_arn = input;
        self
    }
    /// <p>A cost anomaly subscription ARN. </p>
    pub fn get_subscription_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.subscription_arn
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAnomalySubscriptionOutput`](crate::operation::update_anomaly_subscription::UpdateAnomalySubscriptionOutput).
    pub fn build(self) -> crate::operation::update_anomaly_subscription::UpdateAnomalySubscriptionOutput {
        crate::operation::update_anomaly_subscription::UpdateAnomalySubscriptionOutput {
            subscription_arn: self.subscription_arn,
            _request_id: self._request_id,
        }
    }
}
