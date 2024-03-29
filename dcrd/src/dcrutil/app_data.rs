//! App Data Directory Utility
//! Utility to retrieve dcrd/dcrwallet application directory.
use std::{
    env,
    ops::Add,
    path::{Path, PathBuf},
};

/// app_data_dir returns an operating system specific directory to be used for
/// storing application data for an application.
///
/// The `appName` parameter is the name of the application the data directory is
/// being requested for.  This function will prepend a period to the appName for
/// POSIX style operating systems since that is standard practice.  An `empty`
/// appName or one with a single dot is treated as requesting the current
/// directory so only "." will be returned.  Further, the first character
/// of appName will be made lowercase for POSIX style operating systems and
/// uppercase for Mac and Windows since that is standard practice.
///
/// The roaming parameter only applies to Windows where it specifies the roaming
/// application data profile (%APPDATA%) should be used instead of the local one
/// (%LOCALAPPDATA%) that is used by default.
///
/// # Example
///
/// ```
/// let dir = rustdcr::dcrutil::get_app_data_dir("myapp", false);
/// ```
/// ## Gives
///
///   POSIX (Linux/BSD): ~/.myapp
///
///   Mac OS: $HOME/Library/Application Support/Myapp
///
///   Windows: %LOCALAPPDATA%\Myapp
///
///   Plan 9: $home/myapp
pub fn get_app_data_dir(app_name: &str, roaming: bool) -> Option<PathBuf> {
    let dir_data = DirData {
        app_name,
        os: env::consts::OS,
        roaming,
    };

    dir_data.get_app_data_dir()
}

struct DirData<'a> {
    os: &'a str,
    app_name: &'a str,
    roaming: bool,
}

impl<'a> DirData<'a> {
    /// fetch dcrd app directory.
    fn get_app_data_dir(mut self) -> Option<PathBuf> {
        if self.app_name.is_empty() || self.app_name == "." {
            return None;
        }

        // Strip "." if caller prepend a period to path.
        if let Some(e) = self.app_name.strip_prefix('.') {
            self.app_name = e;
        }

        // Get the OS specific home directory.
        match dirs::home_dir() {
            Some(dir) => self.retrieve_from_os(&dir),

            None => match env::var("HOME") {
                Ok(val) => self.retrieve_from_os(Path::new(&val)),

                _ => None,
            },
        }
    }

    /// retrieves app patch using users os attributes.
    fn retrieve_from_os(&self, home_dir: &Path) -> Option<PathBuf> {
        let app_name_upper = self.app_name[..1]
            .to_ascii_uppercase()
            .add(&self.app_name[1..]);

        let app_name_lower = self.app_name[..1]
            .to_ascii_lowercase()
            .add(&self.app_name[1..]);

        match self.os {
            "windows" => {
                // Attempt to use the LOCALAPPDATA or APPDATA environment variable on
                // Windows.
                //
                // Windows XP and before didn't have a LOCALAPPDATA, so fallback
                // to regular APPDATA if LOCALAPPDATA is not set.
                //
                // Since, it is optional to get path on LOCALAPPDATA or APPDATA, error is only captured on LOCALAPPDATA fail.

                if let Ok(mut app_data) = env::var("LOCALAPPDATA") {
                    if app_data.is_empty() || self.roaming {
                        match env::var("APPDATA") {
                            Ok(val) => {
                                app_data = val;
                            }

                            _ => return None,
                        }
                    }

                    return Some(Path::new(&app_data).join(app_name_upper));
                };
            }

            "macos" => {
                if !home_dir.as_os_str().is_empty() {
                    let joined_paths = Path::new(&home_dir)
                        .join("Library")
                        .join("Application Support")
                        .join(app_name_upper);

                    return Some(joined_paths);
                }
            }

            "plan9" => {
                if !home_dir.as_os_str().is_empty() {
                    return None;
                }

                return Some(Path::new(&home_dir).join(app_name_lower));
            }

            _ => {
                if home_dir.as_os_str().is_empty() {
                    return None;
                }

                let mut dotted_path = String::from(".");
                dotted_path.push_str(app_name_lower.as_str());

                return Some(Path::new(&home_dir).join(dotted_path));
            }
        }

        None
    }
}
