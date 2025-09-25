#![allow(clippy::if_same_then_else, clippy::module_inception, clippy::unit_arg)]

#[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

yazi_macro::mod_pub!(app cmp confirm help input mgr notify pick spot tasks which);

yazi_macro::mod_flat!(dispatcher executor logs panic root router signals term);

/// Library function to launch Yazi file manager with given arguments
pub async fn launch_yazi(args: Vec<String>) -> anyhow::Result<()> {
    // Initialize panic handler
    panic::Panic::install();
    yazi_shared::init();

    // Start logging
    logs::Logs::start()?;
    _ = fdlimit::raise_fd_limit();

    // Initialize all Yazi components
    yazi_term::init();
    yazi_fs::init();
    yazi_config::init()?;
    yazi_adapter::init()?;
    yazi_boot::init();
    yazi_proxy::init();
    yazi_dds::init();
    yazi_widgets::init();
    yazi_watcher::init();
    yazi_plugin::init()?;

    // Start DDS and run the app
    yazi_dds::serve();
    app::App::serve().await
}

/// Synchronous wrapper for launching Yazi
pub fn launch_yazi_sync(args: Vec<String>) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(launch_yazi(args))
}