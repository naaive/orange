#[cfg(windows)]
extern crate embed_resource;

fn main() {
  if cfg!(target_os = "macos") {
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.13");
  }

  if cfg!(target_os = "windows") {
    let mut windows = tauri_build::WindowsAttributes::new();
    windows = windows.app_manifest(
      r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <dependency>
        <dependentAssembly>
        </dependentAssembly>
      </dependency>
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                    </requestedPrivileges>
                </security>
            </trustInfo>
            </assembly>
        "#,
    );

    tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
      .expect("failed to run build script");
  } else {
    tauri_build::build()
  }
}
