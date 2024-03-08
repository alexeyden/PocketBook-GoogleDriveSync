pub use sys_stubs::*;

const CONFIG_PATH: &'static str = "/mnt/ext1/system/config/google-drive-sync.json";

pub fn main() {
    inkview::start(async {
        let config = match gdrive_api::ApiConfig::load(CONFIG_PATH) {
            Ok(c) => c,
            Err(e) => {
                inkview::show_message_info("Failed to read config", &e.to_string()).await;
                return;
            }
        };

        let mut api = gdrive_api::GoogleDriveApi::new(config);
        let mut hg = inkview::Hourglass::show("Connecting...").await;

        let root_list = loop {
            match api.list_files("'root' in parents") {
                Ok(list) => break list,
                Err(e) => {
                    if e.as_http_status() == Some(401) {
                        hg.update_text("Failed to authorize, refreshing access token..")
                            .await;

                        if let Err(e) = api.auth_refresh() {
                            inkview::show_message_error(
                                "Failed refresh access token",
                                &e.to_string(),
                            )
                            .await;
                            return;
                        }

                        let _ = api.config().save(CONFIG_PATH);
                        continue;
                    }

                    inkview::show_message_error("Failed to authorize", &e.to_string()).await;
                    return;
                }
            }
        };

        let sync_dir = match root_list.iter().find(|it| it.name == "PocketBookSync") {
            Some(dir) => dir,
            None => {
                inkview::show_message_error(
                    "No sync directory",
                    "'PocketBookSync' folder not found in the root",
                )
                .await;
                return;
            }
        };

        hg.update_text("Fetching file list..").await;

        let sync_list = match api.list_files(&format!("'{}' in parents", sync_dir.id)) {
            Ok(list) => list,
            Err(e) => {
                inkview::show_message_error("Failed to list sync files", &e.to_string()).await;
                return;
            }
        };

        let target_dir = std::path::PathBuf::from("/mnt/ext1/Books/Google Drive");

        if let Err(e) = std::fs::create_dir(&target_dir) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                inkview::show_message_error("Failed to create target dir", &e.to_string()).await;
                return;
            }
        }

        let mut n = 0;

        for file in sync_list {
            let target_file = target_dir.join(&file.name);
            let Some(size) = file.size else {
                continue;
            };

            if !target_file.exists() {
                hg.update_text(&format!("Downloading {}...", file.name))
                    .await;

                let mut temp_file_name = target_file.extension().unwrap().to_owned();
                temp_file_name.push(".part");

                let temp_file = target_file.with_extension(temp_file_name);

                let mut dl = match api.start_download(&file.id, &temp_file) {
                    Ok(dl) => dl,
                    Err(e) => {
                        inkview::show_message_error("Failed to download file", &e.to_string())
                            .await;
                        let _ = std::fs::remove_file(&temp_file);
                        return;
                    }
                };

                let mut prev_percent = 0;

                loop {
                    match dl.next_chunk() {
                        Ok(true) => {
                            let _ = std::fs::rename(temp_file, target_file);
                            n += 1;
                            break;
                        }
                        Ok(false) => {
                            let percent = dl.ready() * 100 / size;

                            if prev_percent != percent {
                                hg.update_text(&format!(
                                    "Downloading {}... ({:02}%)",
                                    file.name, percent
                                ))
                                .await;
                                prev_percent = percent;
                            } else {
                                inkview::sched_point().await;
                            }
                        }
                        Err(e) => {
                            inkview::show_message_error(
                                "Failed to download next chunk",
                                &e.to_string(),
                            )
                            .await;
                            let _ = std::fs::remove_file(&temp_file);
                            return;
                        }
                    }
                }
            }
        }

        inkview::show_message_info("Finished", &format!("Downloaded {} files", n)).await;
    })
}
