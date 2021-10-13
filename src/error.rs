use reqwest::{StatusCode, Url};
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;
use tokio::task::JoinError;

pub type Result<T> = core::result::Result<T, InstallerError>;

#[derive(Error, Debug)]
pub enum InstallerError {
    #[error("Input/Output error")]
    IoError(#[from] std::io::Error),
    #[error("File matcher error")]
    FileMatcherError(#[from] file_matcher::FileMatcherError),
    #[error("Zip error")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Failed to perform a request")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to canonicalize a path {0}")]
    CanonicalizeError(PathBuf, #[source] to_absolute::Error),
    #[error("Failed to serialize as yaml")]
    SerializationAsYamlError(#[from] serde_yaml::Error),
    #[error("Version parse error")]
    ReleaserError(#[from] feenk_releaser::ReleaserError),
    #[error("Walkdir error")]
    WalkdirError(#[from] walkdir::Error),
    #[error("Failed to parse URL")]
    UrlParseError(#[from] url::ParseError),
    #[error("Task join error")]
    JoinError(#[from] JoinError),
    #[error("Mustache template error")]
    MustacheErrorr(#[from] mustache::Error),
    #[error("Failed to detect the latest released version of the gtoolkit-vm from its GitHub repository")]
    FailedToDetectGlamorousAppVersion,
    #[error("Failed to download releaser version from {0}, with status code {1}")]
    FailedToDownloadReleaserVersion(Url, StatusCode),
    #[error("Failed to detect the version of the gtoolkit")]
    FailedToDetectGlamorousImageVersion,
    #[error("Failed to parse the loader {0}")]
    LoaderParseError(String),
    #[error("Workspace already exists: {0}")]
    WorkspaceAlreadyExists(PathBuf),
    #[error("Failed to find the latest release of the Glamorous Toolkit VM")]
    GlamorousToolkitAppIsNotYetReleased,
    #[error("Command {0:?} failed. See install.log or install-errors.log for more info")]
    CommandExecutionFailed(Command),
    #[error("Both private {0:?} and public key {1:?} must be set, or none")]
    SshKeysConfigurationError(Option<PathBuf>, Option<PathBuf>),
    #[error("Specified private key {0} does not exist")]
    PrivateKeyDoesNotExist(PathBuf),
    #[error("Specified public key {0} does not exist")]
    PublicKeyDoesNotExist(PathBuf),
    #[error("Failed to download {0}, status code {1}")]
    DownloadError(Url, StatusCode),
    #[error("Failed to read the file name of {0}")]
    FailedToReadFileName(PathBuf),
    #[error("Failed to read the file extension of {0}")]
    FailedToReadFileExtension(PathBuf),
}

impl<T> From<InstallerError> for std::result::Result<T, InstallerError> {
    fn from(error: InstallerError) -> Self {
        Err(error)
    }
}
