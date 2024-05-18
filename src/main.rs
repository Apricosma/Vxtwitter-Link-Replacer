extern crate clipboard;
extern crate regex;

use clipboard::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting vxtwitter link changer");
    println!("Press Ctrl+C to stop");

    let mut clipboard_ctx: clipboard::windows_clipboard::WindowsClipboardContext = ClipboardProvider::new()
        .expect("Failed to initialize clipboard context");

    let mut last_clipboard_content = String::new();

    // Updated regex to optionally include URL parameters
    let link_pattern = Regex::new(r"^https://(twitter|x)\.com/\w+/status/\d+(\?\S*)?$")
        .expect("Failed to compile regex");

    loop {
        if let Some(modified_link) = check_and_modify_clipboard(&mut clipboard_ctx, &link_pattern, &last_clipboard_content) {
            last_clipboard_content = modified_link;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn check_and_modify_clipboard(clipboard_ctx: &mut ClipboardContext, link_pattern: &Regex, last_content: &str) -> Option<String> {
    match clipboard_ctx.get_contents() {
        Ok(content) if content != *last_content && link_pattern.is_match(&content) => {
            let modified_link = content.replace("twitter.com", "fixupx.com").replace("x.com", "fixupx.com");

            if clipboard_ctx.set_contents(modified_link.clone()).is_ok() {
                println!("Changed link:\nBefore: {}\nAfter: {}", content, modified_link);
                return Some(modified_link);
            }
        }
        _ => {}
    }
    None
}
