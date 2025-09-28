#[cfg(not(debug_assertions))]
mod integration_test {
    use std::path::PathBuf;
    use std::time::{Duration, Instant};

    use db::Database;

    const ENTRIES_ROOT: &str = "/tmp/opl-data/meet-data";

    #[test]
    #[cfg(not(debug_assertions))]
    fn perf_load_data() {
        let path: PathBuf = PathBuf::new().join(ENTRIES_ROOT);
        assert!(path.exists(), "execute`git clone https://gitlab.com/openpowerlifting/opl-data /tmp/opl-data`");
        assert!(path.is_dir());

        let now: Instant = Instant::now();
        Database::from_directory(&path.to_path_buf()).unwrap();
        let elapsed: Duration = now.elapsed();

        assert!(elapsed.as_secs() < 10, "parsing too long: {}ms", elapsed.as_millis());
    }
}
