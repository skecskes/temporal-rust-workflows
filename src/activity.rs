
use log::info;
use anyhow::Error;
use temporal_sdk::ActContext;

use crate::banking_client::BankingService;
use crate::shared::PaymentDetails;

pub async fn withdraw(ctx: ActContext, data: PaymentDetails) -> Result<String, Error> {
  info!("Satrt withdraw of {} from account {}\n\n", data.amount, data.source_account);

  let bank = BankingService {hostname: "bank-api.example.com"};
  return bank.withdraw(&data.source_account, data.amount);
}

pub async fn deposit(ctx: ActContext, data: PaymentDetails) -> Result<String, Error> {
  info!("Start deposit of {} to account {}\n\n", data.amount, data.source_account);

  let bank = BankingService {hostname: "bank-api.example.com"};
  return bank.deposit(&data.source_account, data.amount);
}