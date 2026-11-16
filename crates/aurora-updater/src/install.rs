//! A helper outside the installation performs the directory swap. The old
//! directory remains recoverable until the new process acknowledges a frame.

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command},
    time::{Duration, Instant},
};

const EXE: &str = "SunSet-Viewer.exe";
const ACK_ENV: &str = "AURORA_UPDATE_ACK";
/// Set on the previous viewer restored after a failed update: the
/// transaction folder, whose `failure.txt` says why ([`failed_update`]).
/// Older helpers set it to `1`.
pub const ROLLBACK_ENV: &str = "AURORA_UPDATE_ROLLBACK";
/// Why the update failed, written by the helper before the rollback.
const FAILURE: &str = "failure.txt";
/// Written by the helper just before it starts the new viewer: a failure
/// without it happened while installing, before any start.
const LAUNCHING: &str = "launching";

/// A failed update, told by its helper to the viewer it restored.
pub struct FailedUpdate {
    /// The helper's own words (`failure.txt`).
    pub reason: String,
    /// The new viewer was being started: it failed to start or to show a
    /// frame. Otherwise the installation itself failed and the previous
    /// one was never stopped from starting.
    pub launched: bool,
}
/// Written by the new viewer once its startup splash is on screen: the
/// helper's own update window can then go without a gap.
const SPLASH_SHOWN: &str = "splash-shown";

/// What the helper is doing, for the window the viewer's `main` shows over
/// the whole switch (`splash/`): between the old window's close and the new
/// one's first frame nothing of Aurora was on screen for several seconds
/// (the old process's shutdown, the swap, a cache set aside, the antivirus
/// scan of the new executable).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperStage {
    /// The previous viewer finishes closing.
    Closing,
    /// The installation folders are swapped.
    Installing,
    /// The cache is set aside (a release that asks for an empty cache).
    ClearingCache,
    /// The new viewer was launched and has not shown anything yet.
    Launching,
    /// The new viewer shows its own splash or window: this one goes.
    Handed,
}
const ALLOWED: &[&str] = &[
    EXE,
    "assets",
    "LICENSE",
    "NOTICE.md",
    "README.md",
    "version.txt",
    "aurora-install.json",
    "sunset-install.json",
];

pub struct Workspace {
    pub root: PathBuf,
    install: PathBuf,
    token: String,
    _lock: File,
    handed_off: bool,
    elevate: bool,
}

fn lock_path(install: &Path) -> Result<PathBuf> {
    Ok(install.parent().context("installation parent")?.join(format!(
        ".{}.update.lock",
        install.file_name().context("installation name")?.to_string_lossy()
    )))
}

fn temp_staging_parent() -> Result<PathBuf> {
    let dir = std::env::temp_dir();
    Ok(dir.canonicalize().unwrap_or(dir))
}

fn temp_lock_path(install: &Path) -> Result<PathBuf> {
    let digest = Sha256::digest(install.to_string_lossy().as_bytes());
    let hex: String = digest[..8].iter().map(|b| format!("{b:02x}")).collect();
    Ok(temp_staging_parent()?.join(format!(".aurora-update-{hex}.lock")))
}

fn open_lock_file(path: &Path) -> std::io::Result<File> {
    OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path)
}

fn lock_in_temp(install: &Path) -> Result<File> {
    let file = open_lock_file(&temp_lock_path(install)?)?;
    file.try_lock().context("another updater owns this installation")?;
    Ok(file)
}

fn lock(install: &Path) -> Result<(File, bool)> {
    let install_writable = OpenOptions::new().write(true).open(install.join("LICENSE")).is_ok();
    if install_writable {
        match open_lock_file(&lock_path(install)?) {
            Ok(file) => {
                file.try_lock().context("another updater owns this installation")?;
                return Ok((file, false));
            }
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok((lock_in_temp(install)?, true))
}

fn lock_for_work(install: &Path, work: &Path) -> Result<File> {
    if work.parent() == install.parent() {
        let file = open_lock_file(&lock_path(install)?)?;
        file.try_lock().context("another updater owns this installation")?;
        Ok(file)
    } else {
        lock_in_temp(install)
    }
}

pub(crate) fn plain_dir(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    ensure!(metadata.is_dir() && !metadata.file_type().is_symlink(), "directory is a link");
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        ensure!(metadata.file_attributes() & 0x400 == 0, "directory is a reparse point");
    }
    Ok(())
}

/// The installation sits directly at the root of a drive (`E:\Aurora
/// Viewer`, `/aurora`): its parent has no parent, which `validate_install`
/// refuses, so the update's folder, backup and lock are never written at a
/// drive's root.
pub fn at_drive_root(install: &Path) -> bool {
    install.parent().is_some_and(|p| p.parent().is_none())
}

fn validate_install(install: &Path) -> Result<()> {
    ensure!(
        install.is_absolute() && install.parent().is_some_and(|p| p.parent().is_some()),
        "unsafe installation root: a drive root or a folder directly at a drive root"
    );
    plain_dir(install)?;
    // Never rename a repository, build tree, or a directory with user files.
    for entry in fs::read_dir(install)? {
        let entry = entry?;
        ensure!(
            ALLOWED.iter().any(|n| entry.file_name().eq_ignore_ascii_case(n)),
            "installation contains non-package files"
        );
        ensure!(!entry.file_type()?.is_symlink(), "package entry is a link");
    }
    ensure!(
        install.join(EXE).is_file() && install.join("assets").is_dir() && install.join("LICENSE").is_file(),
        "viewer is not installed as a portable package"
    );
    // A directory junction nested in the assets must never be followed by cleanup.
    validate_tree(&install.join("assets"))?;
    Ok(())
}

pub(crate) fn validate_tree(path: &Path) -> Result<()> {
    plain_dir(path)?;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = fs::symlink_metadata(entry.path())?;
        ensure!(!metadata.file_type().is_symlink(), "package contains a link");
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            ensure!(metadata.file_attributes() & 0x400 == 0, "package contains a reparse point");
        }
        if metadata.is_dir() {
            validate_tree(&entry.path())?;
        }
    }
    Ok(())
}

impl Workspace {
    /// Whether the viewer may update itself in place, and why not.
    pub fn check_install(install: &Path) -> Result<()> {
        validate_install(install)
    }
    pub fn new(install: &Path) -> Result<Self> {
        validate_install(install)?;
        let install = install.canonicalize()?;
        let (mut lock_file, mut elevate) = lock(&install)?;
        sweep_finished(&install);
        let token = uuid::Uuid::new_v4().to_string();
        let dir_name = format!(".aurora-update-{token}");
        let root = if !elevate {
            let candidate = install.parent().context("installation parent")?.join(&dir_name);
            match fs::create_dir(&candidate) {
                Ok(()) => candidate,
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                    elevate = true;
                    drop(lock_file);
                    lock_file = lock_in_temp(&install)?;
                    let fallback = temp_staging_parent()?.join(&dir_name);
                    fs::create_dir(&fallback).context("cannot write to temporary update folder")?;
                    fallback
                }
                Err(e) => return Err(e).context("cannot write next to installation"),
            }
        } else {
            let fallback = temp_staging_parent()?.join(&dir_name);
            fs::create_dir(&fallback).context("cannot write to temporary update folder")?;
            fallback
        };
        Ok(Self {
            root,
            install,
            token,
            _lock: lock_file,
            handed_off: false,
            elevate,
        })
    }

    /// Creates a workspace staged in the user's `%TEMP%` directory even when
    /// the installation directory itself is writable (used by tests and UAC staging).
    pub fn new_in_temp(install: &Path) -> Result<Self> {
        validate_install(install)?;
        let install = install.canonicalize()?;
        let lock_file = lock_in_temp(&install)?;
        let token = uuid::Uuid::new_v4().to_string();
        let root = temp_staging_parent()?.join(format!(".aurora-update-{token}"));
        fs::create_dir(&root).context("cannot write to temporary update folder")?;
        Ok(Self {
            root,
            install,
            token,
            _lock: lock_file,
            handed_off: false,
            elevate: true,
        })
    }

    /// Called on a worker; copies the helper before the viewer shuts down.
    pub fn handoff(mut self, version: &str, maintenance: crate::Maintenance) -> Result<()> {
        let exe = std::env::current_exe()?.canonicalize()?;
        ensure!(exe == self.install.join(EXE), "updater is not running from its installation");
        fs::copy(&exe, self.root.join("helper.exe"))?;
        let args: Vec<String> = std::env::args_os()
            .skip(1)
            .map(|a| a.into_string().map_err(|_| anyhow::anyhow!("non-Unicode restart argument")))
            .collect::<Result<_>>()?;
        let plan = Plan {
            install: self.install.clone(),
            token: self.token.clone(),
            parent: std::process::id(),
            args,
            version: version.into(),
            maintenance,
        };
        let mut file = OpenOptions::new().write(true).create_new(true).open(self.root.join("plan.json"))?;
        file.write_all(&serde_json::to_vec(&plan)?)?;
        file.sync_all()?;
        drop(file);
        if self.elevate {
            spawn_elevated_helper(&self.root)?;
        } else {
            let mut command = Command::new(self.root.join("helper.exe"));
            command.arg("--sunset-update-helper").arg(&self.root).current_dir(&self.root);
            hidden(&mut command);
            let mut child = command.spawn().context("cannot start update helper")?;
            let start = Instant::now();
            while !self.root.join("helper-ready").exists() {
                if child.try_wait()?.is_some() || start.elapsed() >= Duration::from_secs(15) {
                    let _ = child.kill();
                    let _ = child.wait();
                    anyhow::bail!("update helper did not start");
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        self.handed_off = true;
        // Drop the installation lock now: the helper waits for the parent process
        // to exit before taking it and making any change.
        Ok(())
    }
}

#[cfg(windows)]
fn spawn_elevated_helper(root: &Path) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::{
        Foundation::CloseHandle,
        System::Threading::{TerminateProcess, WaitForSingleObject},
        UI::{
            Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW},
            WindowsAndMessaging::SW_HIDE,
        },
    };

    fn wide(s: &std::ffi::OsStr) -> Vec<u16> {
        s.encode_wide().chain(std::iter::once(0)).collect()
    }

    struct HandleGuard(windows_sys::Win32::Foundation::HANDLE);
    impl Drop for HandleGuard {
        fn drop(&mut self) {
            // SAFETY: uniquely owned process handle from ShellExecuteExW.
            unsafe {
                CloseHandle(self.0);
            }
        }
    }

    let verb = wide(std::ffi::OsStr::new("runas"));
    let file = wide(root.join("helper.exe").as_os_str());
    let params_str = format!("--sunset-update-helper \"{}\"", root.display());
    let params = wide(std::ffi::OsStr::new(&params_str));
    let dir = wide(root.as_os_str());
    // SAFETY: SHELLEXECUTEINFOW is plain-old-data and initialized before use.
    let mut info: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    info.fMask = SEE_MASK_NOCLOSEPROCESS;
    info.lpVerb = verb.as_ptr();
    info.lpFile = file.as_ptr();
    info.lpParameters = params.as_ptr();
    info.lpDirectory = dir.as_ptr();
    info.nShow = SW_HIDE;
    // SAFETY: all wide buffers remain alive across the synchronous ShellExecuteExW call.
    let ok = unsafe { ShellExecuteExW(&mut info) };
    ensure!(
        ok != 0 && !info.hProcess.is_null(),
        "administrator permission was declined or update helper failed to start"
    );
    let guard = HandleGuard(info.hProcess);
    let start = Instant::now();
    while !root.join("helper-ready").exists() {
        // SAFETY: guard.0 is a valid process handle.
        let exited = unsafe { WaitForSingleObject(guard.0, 0) == 0 };
        if exited || start.elapsed() >= Duration::from_secs(15) {
            // SAFETY: guard.0 is a valid process handle.
            unsafe {
                let _ = TerminateProcess(guard.0, 1);
            }
            anyhow::bail!("elevated update helper did not start");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

#[cfg(not(windows))]
fn spawn_elevated_helper(_root: &Path) -> Result<()> {
    anyhow::bail!("elevated update helper is Windows-only")
}

impl Drop for Workspace {
    fn drop(&mut self) {
        if !self.handed_off {
            let root = self.root.clone();
            let install = self.install.clone();
            let token = self.token.clone();
            std::thread::spawn(move || {
                if let Err(e) = cleanup_work(&root, &install, &token) {
                    log::warn!("update staging cleanup: {e:#}");
                }
            });
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Plan {
    install: PathBuf,
    token: String,
    parent: u32,
    args: Vec<String>,
    version: String,
    #[serde(default)]
    maintenance: crate::Maintenance,
}

fn validate_work(work: &Path, plan: &Plan) -> Result<()> {
    ensure!(uuid::Uuid::parse_str(&plan.token).is_ok(), "invalid transaction token");
    let temp_parent = temp_staging_parent().ok();
    let valid_parent = work.parent() == plan.install.parent() || (temp_parent.is_some() && work.parent() == temp_parent.as_deref());
    ensure!(
        work.is_absolute()
            && valid_parent
            && work
                .file_name()
                .is_some_and(|n| n == format!(".aurora-update-{}", plan.token).as_str()),
        "unsafe transaction directory"
    );
    plain_dir(work)?;
    ensure!(
        plan.install.parent().is_some_and(|p| p.parent().is_some()) && plan.install != work,
        "unsafe installation directory"
    );
    Ok(())
}

fn cleanup_work(work: &Path, install: &Path, token: &str) -> Result<()> {
    let plan = Plan {
        install: install.into(),
        token: token.into(),
        parent: 0,
        args: vec![],
        version: String::new(),
        maintenance: Default::default(),
    };
    validate_work(work, &plan)?;
    validate_tree(work)?;
    fs::remove_dir_all(work)?;
    Ok(())
}

/// A finished transaction: failed (`failure.txt`) or committed. Its folder
/// holds a whole copy of the viewer, the failed one included, and nothing
/// resumes it. One whose `backup` is still there is never removed: the
/// restoration failed and that backup is the previous installation.
fn finished(work: &Path) -> bool {
    (work.join(FAILURE).exists() || work.join("committed").exists()) && !work.join("backup").exists()
}

/// The folders earlier updates left beside the installation, removed under
/// the installation lock the caller holds.
fn sweep_finished(install: &Path) {
    let Some(Ok(entries)) = install.parent().map(fs::read_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(token) = name.to_str().and_then(|n| n.strip_prefix(".aurora-update-")) else {
            continue;
        };
        let work = entry.path();
        if uuid::Uuid::parse_str(token).is_ok()
            && finished(&work)
            && let Err(e) = cleanup_work(&work, install, token)
        {
            log::warn!("update: an earlier update's folder is left ({e:#})");
        }
    }
}

/// Why the update that restored this viewer failed, when it did; its
/// transaction folder is then removed in the background, once the helper
/// that restored the viewer has exited. A helper of an older version only
/// says that it failed (`None`).
pub fn failed_update() -> Option<FailedUpdate> {
    let work = PathBuf::from(std::env::var_os(ROLLBACK_ENV)?);
    let result = (|| -> Result<(FailedUpdate, Plan)> {
        let plan: Plan = serde_json::from_slice(&fs::read(work.join("plan.json"))?)?;
        validate_work(&work, &plan)?;
        ensure!(
            std::env::current_exe()?.canonicalize()? == plan.install.join(EXE),
            "rollback notice for another installation"
        );
        let reason = fs::read_to_string(work.join(FAILURE))?;
        let failed = FailedUpdate {
            reason: reason.trim().chars().take(400).collect(),
            launched: work.join(LAUNCHING).exists(),
        };
        Ok((failed, plan))
    })();
    match result {
        Ok((failed, plan)) => {
            let when = if failed.launched { "at its start" } else { "while installing" };
            log::warn!(
                "update: the previous version was restored, the new one failed {when}: {}",
                failed.reason
            );
            std::thread::spawn(move || {
                for _ in 0..120 {
                    if !finished(&work) {
                        return;
                    }
                    if let Ok(_lock) = lock(&plan.install)
                        && cleanup_work(&work, &plan.install, &plan.token).is_ok()
                    {
                        return;
                    }
                    // the helper's own executable is in it until it exits
                    std::thread::sleep(Duration::from_millis(500));
                }
                log::warn!("update: the failed update's folder is left for the next update");
            });
            Some(failed)
        }
        Err(e) => {
            log::warn!("update: the previous version was restored, reason unknown ({e:#})");
            None
        }
    }
}

fn hidden(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    #[cfg(not(windows))]
    {
        let _ = command;
    }
}

fn is_cross_device(err: &std::io::Error) -> bool {
    #[cfg(windows)]
    if err.raw_os_error() == Some(17) {
        return true;
    }
    #[cfg(unix)]
    if err.raw_os_error() == Some(18) {
        return true;
    }
    err.kind() == std::io::ErrorKind::CrossesDevices
}

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    plain_dir(from)?;
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        let metadata = fs::symlink_metadata(&src)?;
        ensure!(!metadata.file_type().is_symlink(), "package contains a link");
        if metadata.is_dir() {
            copy_tree(&src, &dst)?;
        } else {
            fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

fn move_dir(from: &Path, to: &Path) -> Result<()> {
    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(e) if is_cross_device(&e) => {
            validate_tree(from)?;
            if let Err(copy_err) = copy_tree(from, to) {
                let _ = fs::remove_dir_all(to);
                return Err(copy_err);
            }
            fs::remove_dir_all(from)?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// The private helper route runs before CLI parsing, logging, GPU or settings.
/// `stage` hears each step of the switch, with the new version.
pub fn run_helper(stage: &mut dyn FnMut(&str, HelperStage)) -> Result<bool> {
    let mut args = std::env::args_os().skip(1);
    let first = args.next();
    if !first
        .as_ref()
        .is_some_and(|a| a == "--aurora-update-helper" || a == "--sunset-update-helper")
    {
        return Ok(false);
    }
    let work = PathBuf::from(args.next().context("missing update transaction")?);
    let plan: Plan = serde_json::from_slice(&fs::read(work.join("plan.json"))?)?;
    validate_work(&work, &plan)?;
    ensure!(
        std::env::current_exe()?.canonicalize()? == work.join("helper.exe"),
        "helper outside transaction"
    );
    let parent = Parent::open(plan.parent)?;
    let version = plan.version.clone();
    let mut stage = |s: HelperStage| stage(&version, s);
    fs::write(work.join("helper-ready"), b"ready")?;
    // after the old viewer's go: its wait for this helper never includes
    // the window's start
    stage(HelperStage::Closing);
    parent.wait()?;
    let result = (|| -> Result<()> {
        stage(HelperStage::Installing);
        let _lock = lock_for_work(&plan.install, &work)?;
        validate_install(&plan.install)?;
        validate_install(&work.join("prepared"))?;
        apply(&plan.install, &work, || {
            if plan.maintenance.clear_cache {
                stage(HelperStage::ClearingCache);
            }
            let mut data = crate::user_data::Backup::prepare(&plan.install, &plan.token, plan.maintenance)?;
            stage(HelperStage::Launching);
            let mut command = Command::new(plan.install.join(EXE));
            // a viewer restored by an earlier failure passes its notice on:
            // the new one would open with "the update did not complete"
            command
                .args(&plan.args)
                .env(ACK_ENV, &work)
                .env_remove(ROLLBACK_ENV)
                .current_dir(&plan.install);
            hidden(&mut command);
            fs::write(work.join(LAUNCHING), b"")?;
            let mut child = command.spawn()?;
            let result = await_frame(&mut child, &work, &plan.token, &plan.version, Duration::from_secs(90), &mut || {
                stage(HelperStage::Handed)
            });
            if !matches!(result, Ok(true)) {
                let _ = child.kill();
                let _ = child.wait();
            }
            if result? {
                data.commit();
                Ok(())
            } else {
                anyhow::bail!("updated viewer did not render a frame")
            }
        })
    })();
    // the new viewer shows, or the old one comes back with its notice
    stage(HelperStage::Handed);
    if let Err(error) = result {
        fs::write(work.join(FAILURE), format!("{error:#}"))?;
        // `apply` restored the directory; launch the old viewer with a visible
        // localized rollback notice. Backups are retained if restoration failed.
        if plan.install.join(EXE).is_file() && !work.join("backup").exists() {
            let mut command = Command::new(plan.install.join(EXE));
            command
                .args(&plan.args)
                .env(ROLLBACK_ENV, &work)
                .env_remove(ACK_ENV)
                .current_dir(&plan.install);
            hidden(&mut command);
            command.spawn()?;
        }
    }
    Ok(true)
}

/// The rollback path is shared with tests using an intentionally failing child.
pub fn apply(install: &Path, work: &Path, launch: impl FnOnce() -> Result<()>) -> Result<()> {
    let install = install.canonicalize()?;
    let install = install.as_path();
    let work = work.canonicalize()?;
    let work = work.as_path();
    let token = work
        .file_name()
        .and_then(|n| n.to_str())
        .and_then(|n| n.strip_prefix(".aurora-update-"))
        .context("invalid staging name")?;
    validate_work(
        work,
        &Plan {
            install: install.into(),
            token: token.into(),
            parent: 0,
            args: vec![],
            version: String::new(),
            maintenance: Default::default(),
        },
    )?;
    validate_install(install)?;
    validate_install(&work.join("prepared"))?;
    let backup = work.join("backup");
    ensure!(!backup.exists(), "backup already exists");
    move_dir(install, &backup).context("cannot move old viewer; close other instances")?;
    if let Err(error) = move_dir(&work.join("prepared"), install) {
        move_dir(&backup, install).context("cannot restore old viewer")?;
        return Err(error);
    }
    match launch() {
        Ok(()) => {
            // A successful new viewer is already running. Cleanup errors cannot
            // turn that into a rollback; leave the backup for later cleanup.
            if let Err(e) = fs::write(work.join("committed"), b"ok") {
                log::warn!("update commit marker: {e}");
            }
            if let Err(e) = validate_tree(&backup).and_then(|()| fs::remove_dir_all(&backup).map_err(Into::into)) {
                log::warn!("update backup cleanup: {e:#}");
            }
            Ok(())
        }
        Err(error) => {
            move_dir(install, &work.join("failed")).context("cannot move failed update")?;
            move_dir(&backup, install).context("cannot restore backup")?;
            Err(error)
        }
    }
}

/// `handed` runs once the new viewer's splash shows, or its first frame.
fn await_frame(child: &mut Child, work: &Path, token: &str, version: &str, timeout: Duration, handed: &mut dyn FnMut()) -> Result<bool> {
    let start = Instant::now();
    let expected = format!("{token}\n{version}");
    let mut splash = false;
    loop {
        if child.try_wait()?.is_some() {
            return Ok(false);
        }
        if fs::read_to_string(work.join("first-frame")).is_ok_and(|s| s == expected) {
            if !splash {
                handed();
            }
            return Ok(true);
        }
        if !splash && fs::read_to_string(work.join(SPLASH_SHOWN)).is_ok_and(|s| s == expected) {
            splash = true;
            handed();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(false);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// The new viewer's startup splash is on screen: the helper's update window
/// goes. Called from the splash's reader thread; nothing when the viewer was
/// not started by an update.
pub fn acknowledge_splash() {
    let Some(path) = std::env::var_os(ACK_ENV) else { return };
    let result = (|| -> Result<()> {
        let work = PathBuf::from(path);
        let plan: Plan = serde_json::from_slice(&fs::read(work.join("plan.json"))?)?;
        validate_work(&work, &plan)?;
        ensure!(plan.version == aurora_version::version(), "update acknowledgement mismatch");
        fs::write(work.join(SPLASH_SHOWN), format!("{}\n{}", plan.token, plan.version))?;
        Ok(())
    })();
    if let Err(e) = result {
        log::warn!("update splash acknowledgement: {e:#}");
    }
}

/// Call only after the render thread reports a presented frame. Disk I/O and
/// waiting for the helper's cleanup stay off the UI thread.
pub fn acknowledge_frame() {
    let Some(path) = std::env::var_os(ACK_ENV) else { return };
    std::thread::spawn(move || {
        let result = (|| -> Result<()> {
            let work = PathBuf::from(path);
            let plan: Plan = serde_json::from_slice(&fs::read(work.join("plan.json"))?)?;
            validate_work(&work, &plan)?;
            ensure!(
                plan.version == aurora_version::version() && std::env::current_exe()?.canonicalize()? == plan.install.join(EXE),
                "update acknowledgement mismatch"
            );
            fs::write(work.join("first-frame"), format!("{}\n{}", plan.token, plan.version))?;
            for _ in 0..300 {
                std::thread::sleep(Duration::from_millis(200));
                if work.join("committed").exists()
                    && let Ok(_lock) = lock_for_work(&plan.install, &work)
                    && cleanup_work(&work, &plan.install, &plan.token).is_ok()
                {
                    break;
                }
            }
            Ok(())
        })();
        if let Err(e) = result {
            log::warn!("update first-frame acknowledgement: {e:#}");
        }
    });
}

#[cfg(windows)]
struct Parent(windows_sys::Win32::Foundation::HANDLE);
#[cfg(windows)]
impl Parent {
    fn open(id: u32) -> Result<Self> {
        use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_SYNCHRONIZE};
        // SAFETY: a PID is passed by our parent; the returned owned handle is
        // used only to wait, never to inspect process memory or credentials.
        let handle = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, 0, id) };
        ensure!(!handle.is_null(), "cannot wait for parent process");
        Ok(Self(handle))
    }
    fn wait(&self) -> Result<()> {
        use windows_sys::Win32::System::Threading::WaitForSingleObject;
        // SAFETY: the owned handle stays alive throughout the bounded wait.
        ensure!(unsafe { WaitForSingleObject(self.0, 120_000) } == 0, "parent viewer did not exit");
        Ok(())
    }
}
#[cfg(windows)]
impl Drop for Parent {
    fn drop(&mut self) {
        // SAFETY: this is the uniquely owned OpenProcess handle.
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}
#[cfg(not(windows))]
struct Parent;
#[cfg(not(windows))]
impl Parent {
    fn open(_: u32) -> Result<Self> {
        anyhow::bail!("automatic installation is Windows-only")
    }
    fn wait(&self) -> Result<()> {
        anyhow::bail!("automatic installation is Windows-only")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_directly_at_a_drive_root_is_told_apart() {
        assert!(at_drive_root(Path::new("/Aurora Viewer")));
        assert!(!at_drive_root(Path::new("/games/Aurora Viewer")));
        assert!(!at_drive_root(Path::new("/")));
        #[cfg(windows)]
        {
            assert!(at_drive_root(Path::new(r"E:\Aurora Viewer")));
            assert!(!at_drive_root(Path::new(r"E:\Games\Aurora Viewer")));
        }
    }

    #[test]
    fn only_finished_updates_without_a_backup_are_swept() {
        let root = std::env::temp_dir().join(format!("aurora-sweep-test-{}", uuid::Uuid::new_v4()));
        let install = root.join("Aurora Viewer");
        fs::create_dir_all(&install).expect("install");
        let work = |marker: &[&str]| {
            let dir = root.join(format!(".aurora-update-{}", uuid::Uuid::new_v4()));
            fs::create_dir_all(dir.join("failed")).expect("work");
            fs::write(dir.join("failed").join(EXE), b"new").expect("copy");
            for name in marker {
                if *name == "backup" {
                    fs::create_dir(dir.join(name)).expect("backup");
                } else {
                    fs::write(dir.join(name), b"x").expect("marker");
                }
            }
            dir
        };
        let failed = work(&[FAILURE]);
        let committed = work(&["committed"]);
        let unrestored = work(&[FAILURE, "backup"]);
        let running = work(&["plan.json"]);
        let canonical = install.canonicalize().expect("canonical");
        sweep_finished(&canonical);
        assert!(!failed.exists() && !committed.exists());
        assert!(unrestored.join("backup").exists(), "the previous installation stays");
        assert!(running.exists(), "a transaction in progress stays");
        fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn a_folder_at_a_drive_root_is_refused_with_that_reason() {
        let error = validate_install(Path::new("/Aurora Viewer")).expect_err("refused");
        assert!(error.to_string().contains("drive root"), "{error:#}");
    }
}
