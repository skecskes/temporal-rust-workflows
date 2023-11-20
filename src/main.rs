use anyhow::{self, bail};
use money_transfer_project_template_rust::shared::{
    PaymentDetails, MONEY_TRANSFER_TASK_QUEUE_NAME, NAMESPACE, WORKFLOW_NAME,
};
use std::sync::Arc;
use temporal_client::{
    ClientOptionsBuilder, WfClientExt, WorkflowClientTrait, WorkflowExecutionResult,
    WorkflowOptions,
};
use temporal_sdk_core::Url;
mod banking_client;
use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;

use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

use actix_web::{
    post,
    web::{self},
    App, HttpResponse, HttpServer, ResponseError, Result,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum WorkflowError {
    #[display(fmt = "Workflow Failure")]
    WorkflowFailure,
}

impl ResponseError for WorkflowError {
    fn error_response(&self) -> HttpResponse {
        match self {
            WorkflowError::WorkflowFailure => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[post("/transfer")]
pub async fn transfer(input: web::Json<PaymentDetails>) -> Result<HttpResponse, WorkflowError> {
    match start_workflow(input.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Success!")),
        Err(_) => Err(WorkflowError::WorkflowFailure),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    return HttpServer::new(|| {
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .wrap(actix_web::middleware::Logger::default())
            .service(transfer)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await;
}

pub enum WorkflowResult {
    Success,
    Failure(String),
}

pub async fn start_workflow(input: PaymentDetails) -> Result<WorkflowResult, anyhow::Error> {
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

    info!(
        "Starting transfer of {} from account {} to account {}",
        input.amount, input.source_account, input.target_account
    );

    let wf_exec_res = client
        .start_workflow(
            vec![input.as_json_payload().expect("serializes fine")],
            MONEY_TRANSFER_TASK_QUEUE_NAME.to_string(),
            WORKFLOW_NAME.to_string(),
            WORKFLOW_NAME.to_string(),
            None,
            WorkflowOptions::default(),
        )
        .await;

    match wf_exec_res {
        Ok(wf_res) => {
            let handle = client.get_untyped_workflow_handle(WORKFLOW_NAME, wf_res.run_id);
            if let WorkflowExecutionResult::Succeeded(_) = handle
                .get_workflow_result(Default::default())
                .await
                .unwrap()
            {
                info!("Workflow executed sucessfully")
            } else {
                error!("Workflow failed");
                return Ok(WorkflowResult::Failure("Failed!".to_owned()));
            }
        }
        Err(status) => {
            bail!("Error executing workflow: {}", status);
        }
    }

    return Ok(WorkflowResult::Success);
}
