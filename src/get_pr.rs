use reqwest::blocking::Client;
use std::env;

pub fn get_pr_body(
    pr_number: u32,
    owner: &str,
    repo: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/files",
        owner, repo, pr_number
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-github-action")
        .bearer_auth(token)
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        if let Some(body) = json.get("body") {
            return Ok(body.as_str().unwrap_or("").to_string());
        }
    }

    Err("Failed to get PR body".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn test_get_pr_body_success() {
        // Set up a mock response
        let _m = mock("GET", "/repos/owner/repo/pulls/1")
            .match_header("User-Agent", "rust-github-action")
            .with_header("Content-Type", "application/json")
            .with_body(r#"{"body": "This is the pull request body."}"#)
            .create();

        env::set_var("GITHUB_TOKEN", "fake_token");
        let result = get_pr_body(1, "dericko681", "fibbot");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_pr_body_not_found() {
        // Set up a mock response for a 404 error
        let _m = mock("GET", "/repos/owner/repo/pulls/2")
            .with_status(404)
            .create();

        env::set_var("GITHUB_TOKEN", "fake_token");

        let result = get_pr_body(2, "dericko681", "fibbot");

        // Assert the result
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Failed to get PR body");
    }

    #[test]
    fn test_get_pr_body_no_body() {
        let _m = mock("GET", "/repos/owner/repo/pulls/3")
            .with_body(r#"{}"#)
            .create();

        env::set_var("GITHUB_TOKEN", "fake_token");

        let result = get_pr_body(3, "dericko681", "fibbot");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ""); // Should return an empty string
    }

    #[test]
    fn test_get_pr_body_missing_token() {
        // Test case where the GitHub token is not set
        env::remove_var("GITHUB_TOKEN");

        let result = get_pr_body(1, "dericko681", "fibbot");

        // Assert the result
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("environment variable"));
    }
}
