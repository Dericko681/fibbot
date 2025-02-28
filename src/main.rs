mod extract;
mod fibonacci;
mod get_pr;
mod post_comment;
use extract::extract_numbers;
use fibonacci::fibonacci_up_to;
use get_pr::get_pr_body;
use post_comment::post_comment;

fn main() {
    // Example values
    let owner = "dericko681";
    let repo = "fibbot";
    let pr_number: u32 = 1;


    match get_pr_body(pr_number, owner, repo) {
        Ok(content) => {
            let extracted_numbers = extract_numbers(content);
            println!("Extracted numbers: {:?}", extracted_numbers);
            for number in extracted_numbers {
                let fibonacci_results = fibonacci_up_to(number);
                println!("The fibo of {} is {:?}", number, fibonacci_results);
                let comment_body = format!(
                    "Fibonacci numbers up to {}: {:?}",
                    number, fibonacci_results
                );
                println!("the comments is:{}", comment_body);
                println!("The fibonacci of {} is: {:?}", number,fibonacci_up_to(number));
                let _ = post_comment(pr_number, owner, repo, comment_body);
               
                // match post_comment(pr_number, owner, repo, comment_body) {
                //     Ok(_) => println!("Comment posted successfully!"),
                //     Err(e) => eprintln!("Error posting comment: {}", e),
                // }
            }
        }
        Err(e) => eprintln!("Error fetching PR body: {}", e),
    }
}
