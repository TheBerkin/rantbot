# RantBot

Simple Discord bot for interpreting Rant programs.

## Building

Install Cargo and run:

```
cargo build
```

This should produce an executable file in the `target` directory.

## Running

1. Assign your app's client secret to an environment variable called `RANTBOT_TOKEN`.
2. Run the executable directly or run `cargo run`.

## Usage

Post a message with a fenced code block with the language ID `rantbot` and RantBot will try to run it.

Depending on the outcome, RantBot will rate the trigger message with one of the following emoji:

|Emoji|Meaning|
|:---:|-------|
|✅|Program ran successfully.|
|❓|The program failed to build due to syntax error(s).|
|❌|The program encountered a runtime error.|

The bot will then reply with the results (or errors), tagging the user who sent the request.

## Minimum Rust version

The app was developed using the 1.47.0 toolchain but might compile under some older versions.