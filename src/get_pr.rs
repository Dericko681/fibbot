use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;

pub async fn get_pr_body(repo_owner: &str, repo_name: &str, pr_number: u64) -> Result<String, Box<dyn std::error::Error>> {
    let octocrab = Octocrab::default();
    let pr: PullRequest = octocrab.pulls(repo_owner, repo_name)
                                   .get(pr_number)
                                   .await?;

    Ok(pr.body.unwrap_or_default())  // Returning the body of the PR
}