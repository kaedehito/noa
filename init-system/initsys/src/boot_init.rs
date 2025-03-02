use anyhow::Result;
use futures::future::join_all;
use std::{fs::read_to_string, path::PathBuf, process::Stdio};
use tokio::{io::AsyncReadExt, process::Command};

use crate::{boot_struct, failed, info, wait, write_to_log};

const VERSION: &'static str = include_str!("../../../noa-version");

pub async fn boot_linux() {
    println!("noa v{}", VERSION);

    let services = get_services().unwrap_or_else(|e| {
        failed!("!!!!! Failed to boot noa services !!!!!");
        failed!("{e}");
        std::process::exit(1);
    });

    let max_name_length = services.iter().map(|s| s.name.len()).max().unwrap_or(0);
    let padding = 5;
    let desc_start_pos = 4 + max_name_length + padding;

    let handles: Vec<_> = services
        .into_iter()
        .map(|service| {
            tokio::spawn(async move {
                wait!("{:<desc_start_pos$}{}", service.name, service.description);

                let args: Vec<String> = service.service.execstart.split_whitespace().map(String::from).collect();
                if args.is_empty() {
                    failed!("Invalid execstart command for {}", service.name);
                    return;
                }

                match Command::new(&args[0])
                    .args(&args[1..])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(mut process) => {
                        let status = process.wait().await.unwrap_or_else(|e| {
                            failed!("Failed to boot {}: {e}", service.name);
                            Default::default()
                        });

                        if status.success() {
                            info!("{:<desc_start_pos$}{}", service.name, service.description);
                        } else {
                            failed!(
                                "{:<desc_start_pos$}Failed to start {}. See /etc/noa/logs/service/{}.log",
                                service.name,
                                service.name,
                                service.name
                            );

                            let mut stdout = String::new();
                            let mut stderr = String::new();

                            if let Some(mut out) = process.stdout {
                                out.read_to_string(&mut stdout).await.unwrap_or_else(|_| {
                                    failed!("Failed to read {}'s stdout", service.name);
                                    0
                                });
                            }

                            if let Some(mut err) = process.stderr {
                                err.read_to_string(&mut stderr).await.unwrap_or_else(|_| {
                                    failed!("Failed to read {}'s stderr", service.name);
                                    0
                                });
                            }

                            let log_content = format!(
                                "======== stdout ========\n{}\n======== stderr ========\n{}",
                                stdout, stderr
                            );
                            write_to_log!(service.name, log_content);
                        }
                    }
                    Err(e) => {failed!("Failed to execute {}: {e}", service.name);},
                }
            })
        })
        .collect();

    join_all(handles).await;
}

pub fn get_services() -> Result<Vec<boot_struct::Noaboot>> {
    let services_path = PathBuf::new()
        .join("/")
        .join("etc")
        .join("noa")
        .join("services");

    let mut services: Vec<boot_struct::Noaboot> = Vec::new();

    for service in services_path.read_dir()? {
        let service = service?;

        let content = read_to_string(service.path())?;

        let serv: boot_struct::Noaboot = toml::from_str(&content)?;
        services.push(serv);
    }

    Ok(services)
}
