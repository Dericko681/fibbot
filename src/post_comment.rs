use octocrab::Octocrab;
use octocrab::models::Comment;


async fn comment_on_pr(repo_owner: &str, repo_name: &str, pr_number: u64, comment: &str) -> Result<Comment, Box<dyn std::error::Error>> {
    let octocrab = Octocrab::default();
    let comment_response = octocrab.pulls(repo_owner, repo_name)
                                    .comments(pr_number)
                                    .create(comment)
                                    .await?;
                                    
    Ok(comment_response)
}
