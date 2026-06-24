use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dashboard/"]
pub struct DashboardAssets;
