Simplistic Google Drive files downloader for PocketBook Pro 912 e-readers written in Rust. Only tested on beforementioned device, but will probably work on other PocketBook models of that era.

The app simply enumerates all files in the predefined Google Drive folder (`<Root>/PocketBookSync`) and downloads them to the predefined directory on the device (`/mnt/ext1/Books/Google Drive`). Existing/already downloaded files are simply skipped, so reuploading a different file with the same name would not trigger download.

# Building & setup instructions

Pre-requirements:

* Modern version of the gcc toolchain for armv5te (`-march=armv5te -mfpu=vfp -mfloat-abi=softfp`). Toolchain from the original SDK is too old to compile `rustls`/`ring`.
* Original SDK (FRSCSDK) to be used as a sysroot
* Installed `armv5te-unknown-linux-gnueabi` rustc target (`rustup add target armv5te-unknown-linux-gnueabi`)

Project setup:

1. Create a Google Cloud project at https://console.cloud.google.com. Add Google Drive API support when prompted for APIs & services selection.
2. Add a new API key for the project on the "APIs & Services -> Credentials" page. Save your API key.
3. Create a folder with your books on the Google Drive and make it public to anyone who has a link. Save folder ID part of the link (`https://drive.google.com/drive/folders/FOLDER_ID?usp=drive_link`).

Building:

1. Set absoulte paths in `sync-app/.cargo/config.toml` to the location of this project on your system (path to the linker wrapper and sysroot path)
2. Set correct paths in the `sync-app/build.sh` (path to the toolchain)
3. Set `FOLDER_ID` and `API_KEY` variables in the `build.sh` to your folder ID and API key saved during project setup. These variables will be embedded into the app binary.
4. You will also need to change linker name in `sync-app/ldwrap.sh` if your toolchain prefix is different.
5. Build the app with: `cd sync-app && ./build.sh`
6. Copy `GoogleDriveSync.app` to the device at `/mnt/ext1/applications`
7. App is ready to use now
