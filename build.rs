fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(&{
        let mut manifest = sxs_manifest::AssemblyManifest::default();

        // XAML Island requires >= 1903, It can be specified by max_version_tested
        manifest.compatibility.max_version_tested =
            Some(sxs_manifest::manifest::windows_version::WINDOWS_10_1903);
        manifest
            .compatibility
            .supported_os
            .insert(sxs_manifest::manifest::SupportedOS::Windows10);

        manifest.serialize_to_string().unwrap()
    });
    res.set_icon("res/Icon.ico");
    res.compile().unwrap();
}
