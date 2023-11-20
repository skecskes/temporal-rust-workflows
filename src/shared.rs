use serde::{Deserialize, Serialize};

pub const NAMESPACE: &str = "default";

pub const MONEY_TRANSFER_TASK_QUEUE_NAME: &str = "TRANSFER_MONEY_TASK_QUEUE";

pub const WORKFLOW_NAME: &str = "money_transfer";

#[derive(Serialize, Deserialize)]
pub struct PaymentDetails {
  pub source_account: String,
  pub target_account: String,
  pub amount: u32
}
