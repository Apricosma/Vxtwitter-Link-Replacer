extern crate clipboard;
extern crate regex;

use clipboard::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use std::thread;
use std::time::Duration;

fn main() {
    let mut clipboard_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_clipboard_content = String::new(); // Initialize with an empty string

    loop {
        // Check the clipboard for changes
        let clipboard_content = match clipboard_ctx.get_contents() {
            Ok(content) => content,
            Err(_) => {
                // Handle clipboard read errors gracefully (e.g., when clipboard contains non-text data)
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        // Only proceed if the clipboard content has changed and is a string
        if clipboard_content != last_clipboard_content {
            last_clipboard_content = clipboard_content.clone();

            // Define a regex pattern to match Twitter and x.com links with "/status/"
            let link_pattern = Regex::new(r"https://(twitter|x)\.com/([^\s]*)/status/([^\s]*)").unwrap();

            // Check if the clipboard content matches the pattern
            if link_pattern.is_match(&clipboard_content) {
                // Replace the domain in the link
                let modified_link = link_pattern.replace(&clipboard_content, "https://vxtwitter.com/$2/status/$3");

                // Update the clipboard content with the modified link
                clipboard_ctx.set_contents(modified_link.to_string()).unwrap();

                // Print a message to the console
                println!("Changed link:\nBefore: {}\nAfter: {}", clipboard_content, modified_link);
            }
        }

        // Sleep for a while to avoid busy-waiting
        thread::sleep(Duration::from_secs(1));
    }
}
