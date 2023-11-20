use log::info;
use rand::Rng;
use anyhow::Error;


pub struct BankingService <'a> {
  pub hostname: &'a str
}

impl BankingService <'_>{

pub fn withdraw(&self, account_number: &str, amount: u32) -> Result<String, Error> {
  let tx_id = BankingService::generate_transaction_id("W", 10);
  info!("Withdrawing {} from account {}, tranction id: {}", amount, account_number, tx_id);
  Ok(tx_id)
}

pub fn deposit(&self, account_number: &str, amount: u32) -> Result<String, Error> {
  let tx_id = BankingService::generate_transaction_id("D", 10);
  info!("Depositing {} to account {}, tranction id: {}", amount, account_number, tx_id);
  Ok(tx_id)
}

fn generate_transaction_id(prefix: &str, length: u32) -> String {
  let mut rng = rand::thread_rng();
  let allowed_chars: Vec<char> = "0123456789".chars().collect();
  let mut id_arr: Vec<char> = vec![' '; length as usize];

  for i in 0..length as usize {
    id_arr[i] = allowed_chars[rng.gen_range(0..allowed_chars.len())];
  }

  return format!("{}{}", prefix, id_arr.iter().collect::<String>());
}

}