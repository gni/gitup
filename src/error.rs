use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Git is not installed on this system.")]
    GitNotInstalled,

    #[error("Failed to execute command: '{command}'.")]
    CommandExecutionError {
        command: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Command '{command}' failed with exit code {code} and output:\n{stderr}")]
    CommandFailed {
        command: String,
        code: i32,
        stderr: String,
    },

    #[error("Could not detect the operating system or package manager.")]
    PlatformDetectionFailed,

    #[error("Profile '{0}' not found.")]
    ProfileNotFound(String),

    #[error("Failed to read or write the global GitUp configuration file.")]
    GlobalConfigError(#[source] std::io::Error),

    #[error("Failed to serialize or deserialize configuration.")]
    SerializationError(#[from] serde_json::Error),

    #[error("User cancelled the operation.")]
    OperationCancelled,

    #[error("An I/O operation failed.")]
    IoError(#[from] std::io::Error),

    #[error("Could not find the home directory for the current user.")]
    HomeDirectoryNotFound,
}
