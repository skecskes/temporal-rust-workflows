use std::{time::Duration};
use anyhow::anyhow;
use log::{error, info};
use temporal_sdk::{ActivityOptions, WfContext, WorkflowResult};
use temporal_sdk_core_protos::coresdk::{
    activity_result::activity_resolution::Status, activity_result::ActivityResolution,
    AsJsonPayloadExt, FromJsonPayloadExt,
};

use crate::shared::PaymentDetails;

pub async fn money_transfer_wf(ctx: WfContext) -> WorkflowResult<()> {
    let payload = ctx.get_args().into_iter().next().expect("one input");

    let payment_details: PaymentDetails =
        PaymentDetails::from_json_payload(&payload).expect("deserializes fine");

    let withdraw_activity_name = "withdraw_activity";
    let ActivityResolution { status } = ctx
        .activity(ActivityOptions {
            activity_type: withdraw_activity_name.to_string(),
            start_to_close_timeout: Some(Duration::from_secs(5)),
            input: payment_details.as_json_payload().expect("serializes fine"),
            ..Default::default()
        })
        .await;

    match status.expect("some status") {
        Status::Completed(_) => info!("{} completed successfully", withdraw_activity_name),
        Status::Failed(_) | Status::Cancelled(_) | Status::Backoff(_) => {
          error!("Activity {} failed!", withdraw_activity_name);
          return Err(anyhow!("Activity {} failed!", withdraw_activity_name));
        }
    }

    let deposit_activity_name = "deposit_activity";
    let ActivityResolution { status } = ctx
        .activity(ActivityOptions {
            activity_type: deposit_activity_name.to_string(),
            start_to_close_timeout: Some(Duration::from_secs(5)),
            input: payment_details.as_json_payload().expect("serializes fine"),
            ..Default::default()
        })
        .await;

    match status.expect("some status") {
        Status::Completed(_) => info!("{} completed successfully", deposit_activity_name),
        Status::Failed(_) | Status::Cancelled(_) | Status::Backoff(_) => {
          error!("Activity {} failed!", deposit_activity_name);
          return Err(anyhow!("Activity {} failed!", deposit_activity_name));
        }
    }

    Ok(().into())
}
