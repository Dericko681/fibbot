// use reqwest;
// use serde_json::Value;

// pub async fn get_pr_files(
//     owner: &str,
//     repo: &str,
//     pr_number: u32,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/files", owner, repo, pr_number);
//     let response = reqwest::get(url).await?;

// use octocrab::models::repos::Content;

//     if response.status().is_success() {
//         let json: Value = response.json().await?;
//         let files = json["files"].as_array().ok_or("Files not found")?;
//         let file_list = files
//             .iter()
//             .map(|file| file["filename"].as_str().unwrap().to_string())
//             .collect::<Vec<String>>()
//             .join("\n");
//         Ok(file_list)
//     } else {
//         Err(format!("Failed to retrieve PR {}: {}", pr_number, response.status()).into())
//     }
// }
pub async fn get_pr_body(pr_number:u64)-> String{
   let content = octocrab::instance().pulls("Dericko681", "fibbot").list_files(pr_number).await;
    let content = content.unwrap().items.first().unwrap().patch.clone().unwrap();
   content
}