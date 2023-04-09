use chrono::{NaiveDate};
use inquire::{
    error::{InquireResult},
    required, CustomType, DateSelect, Text,
    validator::Validation,
};
use reqwest::blocking::multipart;
use scraper::Html;
use std::string::String;

fn main() -> InquireResult<()> {

    let name = Text::new("Event name?")
        .with_validator(required!("This field is required"))
        .prompt()?;

    let start_time = CustomType::new("Start Time:")
        .with_formatter(&|i: u8| format!("{i}"))
        .with_error_message("Please enter a valid number between 0-23")
        .with_help_message("The earliest hour you can meet in (0-23)")
        .prompt()?;

    let end_time = CustomType::new("End Time:")
        .with_formatter(&|i: u8| format!("{i}"))
        .with_error_message("Please enter a valid number between 0-23")
        .with_help_message("The latest hour you can meet in (0-23)")
        .prompt()?;

    let start_date = DateSelect::new("Start Date")
        .with_validator(|d: NaiveDate| {
            let now = chrono::Utc::now().naive_utc().date();

            if d.lt(&now) {
                Ok(Validation::Invalid("Date cannot be in the past".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .unwrap();

    let end_date = DateSelect::new("End Date")
        .with_validator(move |d: NaiveDate| {
            if d.lt(&start_date) {
                Ok(Validation::Invalid("End date cannot be before start date".into()))
            } else {
                // // if d.ge(&(start_date + chrono::Duration::days(28))) {
                // //     Ok(Validation::Invalid("End date cannot be more than 28 days past start date".into()))
                // // } else {
                Ok(Validation::Valid)
                // }
            }
        })
        .prompt()
        .unwrap();

    create_when2meet(
        name,
        start_date.to_string(),
        end_date.to_string(),
        start_time.to_string(), 
        end_time.to_string(),
    );

    Ok(())
}

fn create_when2meet(
    name: String,
    start_date: String,
    end_date: String,
    start_time: String,
    end_time: String,
) {
    let date_range = get_date_range(start_date, end_date);

    let form = multipart::Form::new()
    .text("NewEventName", name)
    .text("DateTypes", "SpecificDates")
    .text("PossibleDates", date_range)
    .text("NoEarlierThan", start_time)
    .text("NoLaterThan", end_time);

    let client = reqwest::blocking::Client::new();
    let resp = client.post( "https://www.when2meet.com/SaveNewEvent.php").multipart(form).send();
    let data = resp.unwrap().text().unwrap();
    println!("{}", parse_html(data));
}

//Get the w2m url from the response HTML
fn parse_html(html: String) -> String {
    let document = Html::parse_document(&html);
    let body_selector = scraper::Selector::parse("body").unwrap(); 
    let body = document.select(&body_selector).next().unwrap();
    let onload = body.value().attr("onload").unwrap().to_string();

    let split: Vec<String> = onload.split("=").map(|s| s.to_string()).collect();
    let s = &split[1];
    let s = s.replace(&['\"', '\''][..], "");

    "https://www.when2meet.com".to_string() + &s
}

//get the date range from start_date and end_date in a string
fn get_date_range(start_date: String, end_date: String) -> String {
    let start_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();

    let mut dates = Vec::new();
    let mut date = start_date;

    while date <= end_date {
        dates.push(date.to_string());
        date = date + chrono::Duration::days(1);
    }

    dates.join("|")
}