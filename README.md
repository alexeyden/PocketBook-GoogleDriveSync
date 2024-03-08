Simplistic Google Drive files downloader for PocketBook Pro 912 e-readers written in Rust. Only tested on beforementioned device, but will probably work on other PocketBook models of that era.

The app simply enumerates all files in the predefined Google Drive folder (`<Root>/PocketBookSync`) and downloads them to the predefined directory on the device (`/mnt/ext1/Books/Google Drive`). Existing/already downloaded files are simply skipped, so reuploading a different file with the same name would not trigger download.

# Setup and installation

Detailed setup instructions:

1. Download latest builds of the authorization wizard (`auth-wizard`) and the device app (`GoogleDriveSync.app`) from the Releases page of this repo
2. Create a Google Cloud project at https://console.cloud.google.com. Add Google Drive API support when prompted for APIs & services selection.
3. Add OAuth Client ID for the project with "Desktop app" application type. Download client secrets as a json file ("Download JSON" button)
4. Run `auth-wizard` with a path to the client secrets json file as an argument, e.g. `./auth-wizard client_secret.json`
5. The wizard will open a default browser and ask for Google Drive access for the app. Accept everything it asks for.
6. On successful authorization, the wizard will create a `google-drive-sync.json` file which contains Google Drive API access tokens. Be careful with that file because it can be used to access any files on your Google Drive without any further authorization.
7. Copy `google-drive-sync.json` to the device at `/mnt/ext1/system/config/`
8. Copy `GoogleDriveSync.app` to the device at `/mnt/ext1/applications`
9. Create `PocketBookSync` directory at the root of your Google Drive. You may copy your books/files there that need to be synced with the device.
10. App should be ready to use at this point

# Building instructions

Pre-requirements:

* Modern version of the gcc toolchain for armv5te (`-march=armv5te -mfpu=vfp -mfloat-abi=softfp`). Toolchain from the original SDK is too old to compile `rustls`/`ring`.
* Original SDK (FRSCSDK) to be used as a sysroot
* Installed `armv5te-unknown-linux-gnueabi` rustc target (`rustup add target armv5te-unknown-linux-gnueabi`)

Instructions:
1. Build `auth-wizard` for the host machine as usual: `cd auth-wizard && cargo build`
2. Edit absoulte paths `sync-app/.cargo/config.toml` to match your system (path to the linker wrapper and sysroot path)
3. Set correct paths in the `build.sh` (path to the toolchain)
4. You will probably also need to change linker name in `sync-app/ldwrap.sh` if your toolchain prefix is different.
5. Build the app with: `cd sync-app && ./build.sh`

