use chrono::Local;
use clap::Parser;
use std::env;
mod custom_types;
mod gitlab_api;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Project name as defined in the configuration file
    #[clap(short, long)]
    project: String,

    /// Verbose information about the reported task
    #[clap(short, long)]
    comment: String,

    /// Task duration in (m)inutes or (h)hours. Example: 30m or 3h
    #[clap(short, long)]
    time: crate::custom_types::ReportedTime,

    /// Date when the task was performed (yyyy--mm-dd). Defaults to today
    #[clap(short, long, default_value_t = Local::now().format("%Y-%m-%d").to_string().parse::<crate::custom_types::ReportedDate>().unwrap())]
    //date: chrono::DateTime<Local>,
    date: crate::custom_types::ReportedDate,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let gitlab_token = env!("GITLAB_PRIVATE_TOKEN", "$GITLAB_PRIVATE_TOKEN is not set");

    println!("Project: {}", args.project);
    println!("Comment: {}", args.comment);
    println!("Time: {}", args.time);
    println!("Time: {}", args.date);
    println!("Gitlab token: {}", gitlab_token);

    let gitlab_project_details =
        crate::gitlab_api::GitLab::get("24415625", "28", gitlab_token).await?;
    println!("{}", gitlab_project_details.title);
    Ok(())
}
