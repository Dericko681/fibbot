mod extract;
mod fibonacci;
mod get_pr;
mod post_comment;
use extract::extract_numbers;
use fibonacci::fibonacci;
use get_pr::get_pr_body;
use post_comment::post_comment;
use std::env;
#[tokio::main]
async fn main() {
   

    //     // Example values
    //     // let owner = "dericko681";
    //     // let repo = "fibbot";
    //     // let pr_number: u32 = 4;

    let pr_number: u64 = env::var("PR_NUMBER")
    .expect("COULDN'T GET PR_NUMBER")
    .parse::<u64>()
    .expect("invalid pr number");
    

    let pr_numbers = get_pr_body(pr_number).await;
    println!("Extracted numbers: {:?}", pr_numbers);

    let pr_fib = extract_numbers(pr_numbers);



    if pr_fib.is_empty() {
        println!("No numbers found in this pull_request.");
    }
    let mut response =
        String::from("#### Fibonacci output of each number in the pull_request is:\n");
    for &num in &pr_fib {
        let fib = fibonacci(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }
        if let Err(e) = post_comment(&response).await {
            eprintln!("Error posting comment: {}", e);
        }
        // }
        // Err(e) => eprintln!("Error fetching PR body: {}", e),
    }

