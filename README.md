# When2Meet CLI

Rapidly generate When2Meet forms without ever leaving your dev environment. Built with Inquire + Rust <3

![Animated GIF making a demonstration of a questionnaire created with this library. You can replay this recording in your terminal with asciinema play command - asciinema play ./assets/expense_tracker.cast](./demo/W2M%20Demo.gif)

## Installation
Install using Cargo:

```
cargo install w2m
```

## Usage
To generate a When2Meet:

```
w2m
```

Answer the prompts for:

- Title
- Start Time
- End Time
- Start Date
- End Date

## Trivia

- When2Meet doesn't have a public API, but it was extremely easy to reverse engineer.
- The API actually has no bound for the date range, so it's possible to create a year-long When2Meet. This isn't possible from the website, which restricts the range to one month. For this reason, the CLI allows for any date range.
- The entire When2Meet front-end is server-side rendered. The GET and POST requests respond with HTML files.
