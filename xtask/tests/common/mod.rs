//! Scratch directories and Git repositories for integration tests.
#![allow(dead_code, unreachable_pub)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Workspace root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Builds attribution text from pieces so test sources hold no literal.
pub fn j(parts: &[&str]) -> String {
    parts.concat()
}

/// An attribution footer.
pub fn footer() -> String {
    j(&["Gener", "ated with Cla", "ude Code"])
}

/// An attribution trailer.
pub fn trailer() -> String {
    j(&[
        "Co-Auth",
        "ored-By: Cla",
        "ude Opus 5.5 <noreply@",
        "anthropic.com>",
    ])
}

/// A directory removed on drop. Its name contains spaces on purpose.
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "xtask {label} {} {}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A scratch Git repository with hooks disabled.
pub struct Repo {
    pub dir: TempDir,
}

impl Repo {
    pub fn new() -> Self {
        let repo = Self {
            dir: TempDir::new("repo"),
        };
        repo.git(&["init", "-q", "-b", "main"]);
        repo.git(&["config", "user.name", "Policy Tester"]);
        repo.git(&["config", "user.email", "tester@example.org"]);
        repo.git(&["config", "commit.gpgsign", "false"]);
        repo.git(&["config", "core.hooksPath", "/dev/null"]);
        repo
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn run(&self, args: &[&str]) -> Output {
        Command::new("git")
            .arg("-C")
            .arg(self.path())
            .args(args)
            .output()
            .expect("run git")
    }

    pub fn git(&self, args: &[&str]) -> String {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    pub fn write(&self, path: &str, text: &str) {
        let target = self.path().join(path);
        std::fs::create_dir_all(target.parent().expect("parent")).expect("mkdir");
        std::fs::write(target, text).expect("write");
    }

    pub fn commit(&self, message: &str) -> String {
        self.git(&["add", "-A"]);
        self.git(&[
            "commit",
            "-q",
            "--allow-empty",
            "--cleanup=verbatim",
            "-m",
            message,
        ]);
        self.git(&["rev-parse", "HEAD"])
    }
}
