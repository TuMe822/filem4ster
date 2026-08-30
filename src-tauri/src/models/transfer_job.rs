use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransferMode {
    SafeCopy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum VerificationMode {
    Quick,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum JobStatus {
    Created,
    Preflight,
    Ready,
    Running,
    Interrupted,
    Failed,
    TransferComplete,
    Verifying,
    Verified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferJob {
    pub id: String,

    pub source: String,
    pub destination: String,

    pub worker: Option<String>,

    pub mode: TransferMode,
    pub verification: VerificationMode,
    pub status: JobStatus,

    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,

    pub bytes_total: u64,
    pub bytes_transferred: u64,

    pub files_total: u64,
    pub files_transferred: u64,

    pub warnings: u64,
    pub errors: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_job_serializes_to_json() {
        let job = TransferJob {
            id: "job-0001".to_string(),

            source: "/mnt/buffalo".to_string(),
            destination: "/srv/storage/Buffalo".to_string(),

            worker: Some("TinyMonkey".to_string()),

            mode: TransferMode::SafeCopy,
            verification: VerificationMode::Full,
            status: JobStatus::Created,

            created_at: "2026-08-30T10:00:00+03:00".to_string(),
            started_at: None,
            finished_at: None,

            bytes_total: 1_900_000_000_000,
            bytes_transferred: 0,

            files_total: 200_924,
            files_transferred: 0,

            warnings: 0,
            errors: 0,
        };

        let json = serde_json::to_string(&job).expect("TransferJob should serialize");

        assert!(json.contains("\"id\":\"job-0001\""));
        assert!(json.contains("\"mode\":\"safeCopy\""));
        assert!(json.contains("\"worker\":\"TinyMonkey\""));
        assert!(json.contains("\"bytesTotal\":1900000000000"));
    }
}
