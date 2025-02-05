// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeChannelMembership`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::set_channel_arn): <p>The ARN of the channel.</p>
    ///   - [`member_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::member_arn) / [`set_member_arn(Option<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::set_member_arn): <p>The <code>AppInstanceUserArn</code> of the member.</p>
    ///   - [`chime_bearer(impl ::std::convert::Into<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::set_chime_bearer): <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    ///   - [`sub_channel_id(impl ::std::convert::Into<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::sub_channel_id) / [`set_sub_channel_id(Option<String>)`](crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::set_sub_channel_id): <p>The ID of the SubChannel in the request. The response contains an <code>ElasticChannelConfiguration</code> object.</p> <note>   <p>Only required to get a user’s SubChannel membership details.</p>  </note>
    /// - On success, responds with [`DescribeChannelMembershipOutput`](crate::operation::describe_channel_membership::DescribeChannelMembershipOutput) with field(s):
    ///   - [`channel_membership(Option<ChannelMembership>)`](crate::operation::describe_channel_membership::DescribeChannelMembershipOutput::channel_membership): <p>The details of the membership.</p>
    /// - On failure, responds with [`SdkError<DescribeChannelMembershipError>`](crate::operation::describe_channel_membership::DescribeChannelMembershipError)
    pub fn describe_channel_membership(&self) -> crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder {
        crate::operation::describe_channel_membership::builders::DescribeChannelMembershipFluentBuilder::new(self.handle.clone())
    }
}
