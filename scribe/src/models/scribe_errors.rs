use peace::{
    cfg::{AppName, Profile},
    rt_model::fn_graph::{Edge, WouldCycle},
    miette
};


/// Error that would pop up during flows
#[derive(peace::miette::Diagnostic)]
#[derive(Debug, thiserror::Error)]
pub enum ScribeError {
    #[error("Failed to parse pyproject.toml")]
    PyProjectParseError(
        #[source]
        #[from]
        toml::de::Error,
    ),

    #[error("Profile to switch to does not exist.")]
    #[diagnostic(
        code(app_cycle::profile_switch_to_non_existent),
        help(
            "The `{profile_to_switch_to}` profile does not exist."
        )
    )]
    ProfileSwitchToNonExistent {
        /// The profile that the user tried to switch to.
        profile_to_switch_to: Profile,
        /// Name of this app.
        app_name: AppName,
    },

    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[diagnostic_source]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    /// This should technically never happen.
    #[error("A `peace` runtime error occurred.")]
    WouldCycleError(
        #[source]
        #[from]
        WouldCycle<Edge>,
    ),

    // === Scaffolding errors === //
    #[error("Failed to initialize tokio runtime.")]
    TokioRuntimeInit(#[source] std::io::Error),
}