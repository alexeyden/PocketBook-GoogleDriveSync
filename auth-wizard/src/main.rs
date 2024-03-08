fn main() {
    let creds_file = std::env::args().nth(1).expect(&format!(
        "usage: {} oauth-creds.json",
        std::env::args().nth(0).unwrap()
    ));

    let creds_file = std::path::PathBuf::from(creds_file);
    let exported_config =
        gdrive_api::ExportedConfig::import(&creds_file).expect("failed to load creds file");
    let mut api = gdrive_api::GoogleDriveApi::new(exported_config.into());

    println!("Authorizing... You will be prompted for consent in the browser window");
    api.auth().expect("authorization failed");
    api.config()
        .save("google-drive-sync.json")
        .expect("failed to save output config file");

    println!("Success! App config was saved to google-drive-sync.json");
}
