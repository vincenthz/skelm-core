use skelm_ollama as ollama;
use tokio::io::AsyncReadExt;

pub trait ProgressDisplay {
    fn progress_start(size: Option<u64>) -> Self;
    fn progress_update(&self, position: u64);
    fn progress_finalize(self);
}

#[derive(Clone)]
pub struct NoProgress;

impl ProgressDisplay for NoProgress {
    fn progress_start(_size: Option<u64>) -> Self {
        Self
    }

    fn progress_update(&self, _position: u64) {}

    fn progress_finalize(self) {}
}

pub trait DataUpdatable {
    fn ctx_new() -> Self;
    fn ctx_update(&mut self, data: &[u8]);
    fn ctx_update_read_file(
        &mut self,
        file: &mut tokio::fs::File,
    ) -> impl Future<Output = std::io::Result<()>> {
        async {
            // NOTE: use `read` (fills the slice, returns the count) rather than
            // `read_buf` (which appends into a `Vec`'s spare capacity). With a
            // pre-sized `vec![0; N]`, `read_buf` would leave the read bytes at the
            // end while `buf[0..n]` still pointed at the leading zeros — feeding
            // garbage to the hash and corrupting the digest of any resumed download.
            let mut buf = vec![0u8; 16384];
            loop {
                let n = file.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                self.ctx_update(&buf[0..n]);
            }
            Ok(())
        }
    }
}

pub struct DataUpdatableNoop;

impl Default for DataUpdatableNoop {
    fn default() -> Self {
        Self
    }
}

impl DataUpdatable for DataUpdatableNoop {
    fn ctx_new() -> Self {
        Self
    }
    fn ctx_update(&mut self, _data: &[u8]) {}
    async fn ctx_update_read_file(&mut self, _file: &mut tokio::fs::File) -> std::io::Result<()> {
        Ok(())
    }
}

impl DataUpdatable for ollama::BlobContext {
    fn ctx_new() -> Self {
        ollama::BlobContext::new_sha256()
    }

    fn ctx_update(&mut self, data: &[u8]) {
        self.update(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ollama::BlobContext;

    /// Resuming an interrupted download must produce the same digest as hashing
    /// the whole file in one pass: the already-downloaded bytes are replayed
    /// through `ctx_update_read_file`, then the remainder is streamed in.
    #[tokio::test]
    async fn resume_hash_matches_full_hash() {
        // Partial deliberately exceeds the 16 KiB read buffer to exercise the
        // multi-read loop where the old bug corrupted the state.
        let partial = b"the quick brown fox ".repeat(2000); // 40_000 bytes
        let remainder = b"jumps over the lazy dog ".repeat(500);
        let mut full = partial.clone();
        full.extend_from_slice(&remainder);

        // Expected: hash the whole content in one go.
        let mut whole = BlobContext::new_sha256();
        whole.update(&full);
        let expected = whole.finalize();

        // Simulate resume: partial bytes are on disk; replay them, then feed the
        // remainder exactly as the download stream would.
        let path =
            std::env::temp_dir().join(format!("skelm-resume-test-{}.bin", std::process::id()));
        tokio::fs::write(&path, &partial).await.unwrap();
        let mut file = tokio::fs::File::open(&path).await.unwrap();

        let mut resumed = BlobContext::new_sha256();
        resumed.ctx_update_read_file(&mut file).await.unwrap();
        resumed.update(&remainder);
        let resumed = resumed.finalize();

        let _ = tokio::fs::remove_file(&path).await;
        assert!(
            resumed == expected,
            "resumed digest {resumed} != whole-file digest {expected}"
        );
    }
}
