# Temporal Rust Project Template

This is a simple project for demonstrating Temporal with the Rust SDK and Actix Web server.

It is derived from sismilar Go example.

The full 20-minute Go tutorial is here: https://learn.temporal.io/getting_started/go/first_program_in_go/

## Basic instructions

### Step 0: Clone this Repository

In another terminal instance, clone this repo and run this application.

```bash
git clone https://github.com/ivan-mudrak/money-transfer-project-template-rust
cd money-transfer-project-template-rust
```

### Step 1: Run Temporal Server

From the root of the project:

```bash
docker-compose up
```

Leave it running. You can use the Temporal Web UI at [localhost:8080](localhost:8080) which is currently in Beta. To use the legacy Temporal Web UI, use the [localhost:8088](localhost:8088) URL instead. There should be no workflows visible in the dashboard right now.

### Step 2: Start Actix Web server

```bash
cargo run --bin main
```
It will start web-server on localhost:800.

### Step 3: Run the Worker

In ANOTHER terminal instance, run the worker. Notice that this worker hosts both Workflow and Activity functions.

```bash
cargo run --bin worker
```

### Step 4: Execute workflow

In YET ANOTHER terminal instance, send post request using curl. 

```bash
curl --location --request POST 'localhost:8000/transfer' \
--header 'Content-Type: application/json' \
--data-raw '{
    "source_account": "11-11",
    "target_account": "22-22",
    "amount": 100
}'
```

Now you can see the workflow run to completion. You can also see the worker polling for workflows and activities in the task queue at [http://localhost:8080/namespaces/default/task-queues/TRANSFER_MONEY_TASK_QUEUE](http://localhost:8080/namespaces/default/task-queues/TRANSFER_MONEY_TASK_QUEUE).

<img width="882" alt="CleanShot 2021-07-20 at 17 48 45@2x" src="https://user-images.githubusercontent.com/6764957/126413160-18663430-bb7a-4d3a-874e-80598e1fa07d.png">
