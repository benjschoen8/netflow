/// Resolves the SQLite database path for each target.
///
/// Prod (`api`, `ffi`):      `{project-root}/data/netflow.db`
/// Test (`test-api`, `test-ffi`): `{project-root}/backend/netflow-test.db`
///
/// The project root is found by walking upward from the binary until a
/// directory containing both `backend/` and `frontend/` subdirectories is
/// found. Falls back gracefully if the monorepo layout isn't detected.
pub mod db_path {
    use std::path::PathBuf;

    pub fn prod() -> String {
        root().join("data").join("netflow.db")
            .to_string_lossy().into_owned()
    }

    pub fn test() -> String {
        root().join("backend").join("netflow-test.db")
            .to_string_lossy().into_owned()
    }

    fn root() -> PathBuf {
        let exe = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."));
        let mut dir = exe.parent().unwrap_or(exe.as_path()).to_path_buf();
        for _ in 0..8 {
            if dir.join("backend").exists() && dir.join("frontend").exists() {
                return dir;
            }
            match dir.parent() {
                Some(p) => dir = p.to_path_buf(),
                None    => break,
            }
        }
        // Fallback: binary is usually at target/{profile}/bin, so go up three.
        exe.ancestors().nth(3)
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf()
    }
}
