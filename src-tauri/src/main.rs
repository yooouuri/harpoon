// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::service::ContainerSummary;
use bollard::container::{ListContainersOptions, LogsOptions};
use bollard::{Docker, API_DEFAULT_VERSION};
use log::{debug, error};
use tauri::{AppHandle, Manager};
use futures_util::StreamExt;
use tauri_plugin_store::Builder;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{channel, Sender, Receiver};

struct AsyncProcInputTx {
    inner: Mutex<Sender<String>>,
    outer: Mutex<Receiver<String>>,
}

struct Connection(Mutex<Option<Docker>>);

#[tauri::command]
async fn connect(path: String, app_handle: AppHandle, state: tauri::State<'_, Connection>) -> Result<(), String> {
    let mut docker = state.inner().0.lock().await;

    let connection = Docker::connect_with_socket(path.as_str(), 120, API_DEFAULT_VERSION).unwrap();

    // match connection.ping().await {
    //     Ok(_) => debug!("connected to docker"),
    //     Err(e) => return Err(e.to_string()),
    // }

    connection.ping().await.map_err(|e| e.to_string())?;

    *docker = Some(connection);

    Ok(())
}

#[tauri::command]
async fn list_containers(state: tauri::State<'_, Connection>) -> Result<Vec<ContainerSummary>, ()> {
    let docker = state.inner().0.lock().await;
    let docker = docker.as_ref().unwrap();

    let mut filter = HashMap::new();
    filter.insert(String::from("status"), vec![String::from("running")]);

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters: filter,
            ..Default::default()
        }))
        .await.unwrap();

    Ok(containers)
}

// TODO create a command to get x amount of logs from the container since now

#[tauri::command]
async fn container_logs(container_name: String, app_handle: AppHandle, state: tauri::State<'_, Connection>) -> Result<(), ()> {
    let docker = state.inner().0.lock().await;
    let docker = docker.as_ref().unwrap();

    let handle = app_handle.clone();

    let now = chrono::offset::Local::now().timestamp();

    let options = Some(LogsOptions::<String>{
        stdout: true,
        follow: true,
        since: now,
        ..Default::default()
    });

    let mut stream = docker.logs(container_name.as_str(), options);

    let task = tauri::async_runtime::spawn(async move {
        debug!("start container logs thread");

        while let Some(log) = stream.next().await {
            handle.emit_all("container-logs", format!("{}", log.unwrap())).unwrap();
        }
    });

    app_handle.once_global("stop-logs", move |_| {
        task.abort();

        debug!("stopped container logs thread");
    });

    Ok(())
}

#[tauri::command]
async fn run_command(command: String, state: tauri::State<'_, AsyncProcInputTx>) -> Result<(), String> {
    let async_proc_input_tx = state.inner.lock().await;

    async_proc_input_tx
        .send(command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_exec_listener(container_name: String, app_handle: tauri::AppHandle, state: tauri::State<'_, Connection>) -> Result<(), ()> {
    let handle = app_handle.clone();

    let docker = state.inner().0.lock().await;
    let docker = docker.as_ref().unwrap();

    let exec = docker
        .create_exec(
            container_name.as_str(),
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                attach_stdin: Some(true),
                tty: Some(true),
                cmd: Some(vec!["sh"]),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;

    if let StartExecResults::Attached {
        mut output,
        mut input,
    } = docker.start_exec(
        &exec,
        Some(StartExecOptions {
            detach: false,
            ..Default::default()
        })
    ).await.unwrap()
    {
        tauri::async_runtime::spawn(async move {
            while let Some(output) = output.next().await {
                match output {
                    Ok(output) => handle.emit_all("exec-output", format!("{}", output)).unwrap(),
                    Err(e) => error!("error reading: {}", e), // TODO send error to frontend
                }
            }
        });

       tauri::async_runtime::spawn(async move {
            let state = app_handle.state::<AsyncProcInputTx>();

            let mut async_proc_output_rx = state.outer.lock().await;

            while let Some(output) = async_proc_output_rx.recv().await {
                debug!("input from mutex: {}", output);

                match input.write(output.as_bytes()).await {
                    Ok(_) => debug!("wrote to input"),
                    Err(e) => error!("error writing to input: {}", e), // TODO send error to frontend
                };
            }
        });
    }

    Ok(())
}

fn main() {
    env_logger::init();

    let (async_proc_input_tx, mut async_proc_input_rx) = channel::<String>(1);
    let (async_proc_output_tx, async_proc_output_rx) = channel(1);

    tauri::Builder::default()
        .plugin(Builder::default().build())
        .manage(AsyncProcInputTx {
            inner: Mutex::new(async_proc_input_tx),
            outer: Mutex::new(async_proc_output_rx),
        })
        .manage(Connection(Default::default()))
        .setup(|_| {
            tauri::async_runtime::spawn(async move {
                while let Some(input) = async_proc_input_rx.recv().await {
                    let output = input;

                    async_proc_output_tx.send(output).await.ok();
                }
            });

            Ok(())
        })
    .invoke_handler(tauri::generate_handler![connect, list_containers, container_logs, create_exec_listener, run_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
