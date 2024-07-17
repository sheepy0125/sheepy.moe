//! Some page links as `href`s

pub const SENTINEL: &str = "javascript:void(0);";

pub const COUNT_INCREMENT_REDIRECT: &str = "/count-increment-redirect";
pub const SAVE_THEME_CHANGE: &str = "/save-theme-change";

#[cfg(target_arch = "wasm32")]
pub const EXTERNAL_EMAIL: &str = "sheepy@sheepy.moe";
#[cfg(not(target_arch = "wasm32"))]
pub const EXTERNAL_EMAIL: &str = "sheepy(AT@)sheepy(DOT.)moe";
pub const EXTERNAL_MATRIX: &str = "@sheepy:cinderblock.moe";
pub const EXTERNAL_MASTODON: &str = "https://wetdry.world/@sheepy0125";
pub const EXTERNAL_PRONOUNS: &str = "https://pronouns.cc/@sheepy";
