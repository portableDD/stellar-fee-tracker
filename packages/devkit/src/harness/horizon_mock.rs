/// Mock implementation of the Horizon API server for use in tests.
pub struct HorizonMock {
    /// Name of the currently active scenario.
    pub scenario: String,
}

impl HorizonMock {
    pub fn new(scenario: impl Into<String>) -> Self {
        Self { scenario: scenario.into() }
    }

    /// Logs a request to stdout with timestamp, method, path, and active scenario name.
    pub fn log_request(&self, method: &str, path: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        println!("[{}] {} {} scenario={}", now, method, path, self.scenario);
    }

    /// Returns the JSON body for `GET /health`.
    pub fn health_payload(&self) -> String {
        format!(r#"{{"status":"ok","scenario":"{}"}}"#, self.scenario)
    /// Path to the scenario JSON file to serve.
    pub scenario_path: std::path::PathBuf,
}

impl HorizonMock {
    pub fn new(scenario_path: impl Into<std::path::PathBuf>) -> Self {
        Self { scenario_path: scenario_path.into() }
    }

    /// Loads and returns the scenario JSON to be served at `GET /fee_stats`.
    pub fn fee_stats_payload(&self) -> std::io::Result<String> {
        crate::harness::scenarios::load_from_file(&self.scenario_path)
    }
}
