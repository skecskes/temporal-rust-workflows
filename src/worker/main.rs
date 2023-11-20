use std::sync::Arc;
use log::LevelFilter;
use log::error;
use log::info;
use money_transfer_project_template_rust::activity::deposit;
use money_transfer_project_template_rust::activity::withdraw;
use money_transfer_project_template_rust::shared::MONEY_TRANSFER_TASK_QUEUE_NAME;
use money_transfer_project_template_rust::shared::NAMESPACE;
use money_transfer_project_template_rust::shared::WORKFLOW_NAME;
use money_transfer_project_template_rust::workflow::money_transfer_wf;
use simple_logger::SimpleLogger;
use temporal_client::ClientOptionsBuilder;
use temporal_sdk::Worker;
use temporal_sdk_core::TelemetryOptionsBuilder;
use temporal_sdk_core::Url;
use temporal_sdk_core::{init_worker, telemetry_init, WorkerConfig, WorkerConfigBuilder};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let tm_options = TelemetryOptionsBuilder::default().build().unwrap();
    telemetry_init(&tm_options).expect("Telemetry inits cleanly");

    let worker_config: WorkerConfig = WorkerConfigBuilder::default()
        .namespace(NAMESPACE)
        .task_queue(MONEY_TRANSFER_TASK_QUEUE_NAME)
        .worker_build_id("worker_build_id")
        .build()
        .unwrap();

    let client_options = ClientOptionsBuilder::default()
        .client_name("client_name")
        .client_version("0.1.0")
        .target_url(Url::try_from("http://localhost:7233").unwrap())
        .build()
        .unwrap();

    let client = Arc::new(
        client_options
            .connect(NAMESPACE, None, None)
            .await
            .expect("Must connect"),
    );
    let core_worker = init_worker(worker_config, client.clone());

    let mut worker = Worker::new_from_core(Arc::new(core_worker), MONEY_TRANSFER_TASK_QUEUE_NAME);

    worker.register_wf(WORKFLOW_NAME, money_transfer_wf);
    worker.register_activity("withdraw_activity", withdraw);
    worker.register_activity("deposit_activity", deposit);

    match worker.run().await {
        Ok(_) => info!("Worker finished succesfully"),
        Err(e) => error!("Error occur while running worker: {}", e),
    }
}
