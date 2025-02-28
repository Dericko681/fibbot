mod extract; mod fibonacci; mod get_pr; mod post_comment;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve inputs from environment variables
    let repo_owner = env::var("GITHUB_REPOSITORY_OWNER")?;
    let repo_name = env::var("GITHUB_REPOSITORY_NAME")?;
    let pr_number: u64 = env::var("GITHUB_REF").unwrap_or_else(|_| "0".to_string()).parse()?;
    let enable_fibonacci: bool = env::var("INPUT_ENABLE_FIBONACCI").unwrap_or_else(|_| "false".to_string()).parse()?;
    let threshold: u32 = env::var("INPUT_THRESHOLD").unwrap_or_else(|_| "0".to_string()).parse()?;

    // Fetch the pull request body
    let pr_body = get_pr_body(&repo_owner, &repo_name, pr_number).await?;

    // Extract numbers from the PR body
    let numbers = extract_numbers(pr_body.clone());

    let mut results = vec![];

    if enable_fibonacci {
        for &number in &numbers {
            if number <= threshold {
                results.push((number, fibonacci_up_to(number)));
            }
        }
    }

    // Construct the comment to post
    let mut comment = String::from("Fibonacci Results:\n");

    for (num, fibs) in results {
        comment.push_str(&format!("Fibonacci numbers up to {}: {:?}\n", num, fibs));
    }

    // Comment on the PR
    comment_on_pr(&repo_owner, &repo_name, pr_number, &comment).await?;

    Ok(())
}