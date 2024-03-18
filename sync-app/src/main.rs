pub use sys_stubs::*;

pub fn main() {
    const API_KEY: &str =  std::env!("API_KEY");
    const FOLDER_ID: &str = std::env!("FOLDER_ID");

    inkview::start(async {
        let api = gdrive_api::GoogleDriveApi::new(API_KEY.to_owned());
        let mut hg = inkview::Hourglass::show("Fetching file list...").await;

        let root_list = loop {
            match api.list_files(&format!("'{}' in parents", FOLDER_ID)) {
                Ok(list) => break list,
                Err(e) => {
                    inkview::show_message_error("Failed to list files", &e.to_string()).await;
                    return;
                }
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

        for file in root_list {
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
