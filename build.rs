fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    
    // Embed icon on Windows builds
    #[cfg(windows)]
    {
        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
