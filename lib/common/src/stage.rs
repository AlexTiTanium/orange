/// Name of app stage that runs before all other app stages
pub const FIRST: &str = "first";

/// Name of app stage that runs once before the startup stage
pub const PRE_STARTUP: &str = "pre_startup";

/// Name of app stage that runs once when an app starts up
pub const STARTUP: &str = "startup";

/// Name of app stage that runs once after the startup stage
pub const POST_STARTUP: &str = "post_startup";

/// Name of app stage that runs before EVENT
pub const PRE_EVENT: &str = "pre_events";

/// Name of app stage that updates events. Runs before UPDATE
pub const EVENT: &str = "events";

/// Name of app stage responsible for performing setup before an update. Runs before UPDATE.
pub const PRE_UPDATE: &str = "pre_update";

/// Name of app stage responsible for doing most app logic. Systems should be registered here by default.
pub const UPDATE: &str = "update";

/// Name of app stage responsible for processing the results of UPDATE. Runs after UPDATE.
pub const POST_UPDATE: &str = "post_update";

/// When all events cleared
pub const PRE_RENDER: &str = "pre_render";

/// Renderers
pub const RENDER: &str = "render";

/// After render workload
pub const POST_RENDER: &str = "post_render";

/// Name of app stage that runs after all other app stages
pub const LAST: &str = "last";
