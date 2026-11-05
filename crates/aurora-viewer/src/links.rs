//! Project links shown on the login screen. Empty = not configured yet: the
//! button is shown disabled and nothing is fetched.

/// Source code repository (GitHub page).
pub const GITHUB: &str = "https://github.com/Adriano-Severino/second-life-projeto";
/// Issue template chooser, opened in the browser by "Report a bug" (login
/// screen, top bar, Help menu) and by the crash window.
pub const NEW_ISSUE: &str = "https://github.com/Adriano-Severino/second-life-projeto/issues/new";
/// The project's wiki, generated from the guides of `docs/`
/// (`scripts/build-wiki.ps1`).
pub const WIKI: &str = "https://github.com/Adriano-Severino/second-life-projeto/wiki";
/// Not configured until the project has its own community server.
pub const DISCORD: &str = "";
/// The project folder in its source repository, until it has a website.
pub const WEBSITE: &str = "https://github.com/Adriano-Severino/second-life-projeto/tree/main/SunSet-Viewer";
/// Privacy information maintained with this project's source.
pub const PRIVACY_POLICY: &str = "https://github.com/Adriano-Severino/second-life-projeto/blob/main/SunSet-Viewer/README.md#privacy";
/// News feed: this folder of `main`, hand-edited Markdown files, one
/// `<language>.md` per UI language (`SunSet-Viewer/news/`). The viewer keeps
/// what it read in its cache as a JSON array of
/// `{"date", "title", "summary", "url"}`.
pub const NEWS_FEED: &str = "https://raw.githubusercontent.com/Adriano-Severino/second-life-projeto/main/SunSet-Viewer/news/";
/// Files of the source repository as GitHub serves them unrendered; a branch
/// name and a path follow.
pub const RAW_FILES: &str = "https://raw.githubusercontent.com/Adriano-Severino/second-life-projeto/";
