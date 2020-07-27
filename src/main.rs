/// Main file to translate markdown into HTML.
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let file_path = Path::new(filename);
    let file = File::open(&file_path)
               .expect("[ ERROR ] Failed to open file!");
    let reader = BufReader::new(file);

    let html_tokens = convert_to_html(reader);

    let mut output_filename = String::from(&filename[..filename.len()-3]);
    output_filename.push_str(".html");
    write_to_file(output_filename, html_tokens);
    
    println!("[ INFO ] Parsing complete!");
}

fn convert_to_html(reader: BufReader<File>) -> Vec<String> {
    let mut p_tag: bool = false; // is this a paragraph tag
    let mut h1_tag: bool = false; // is this an h1 tag
    let mut html_tokens: Vec<String> = Vec::new();

    for line in reader.lines() {
        // For each line, unwrap it
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut html_line = String::new();
    
        match first_char.pop() {
            Some('#') => {
                // starting an h1 element
                if p_tag {
                    p_tag = false;
                    html_line.push_str("</p>\n");
                }
                if h1_tag {
                    html_line.push_str("</h1>\n");
                }
    
                h1_tag = true;
                html_line.push_str("\n<h1>");
                html_line.push_str(&line_contents[2..]);
            },
            _ => {
                if !p_tag {
                    p_tag = true; 
                    html_line.push_str("<p>");
                }

                html_line.push_str(&line_contents);
            }
        }

        if p_tag {
            p_tag = false;
            html_line.push_str("</p>\n");
        }
        if h1_tag {
            h1_tag = false;
            html_line.push_str("</h1>\n");
        }

        if html_line != "<p></p>\n" {
            html_tokens.push(html_line);
        }
    }
    html_tokens
}

fn write_to_file(output_filename: String, html_tokens: Vec<String>) {
    let mut outfile = File::create(&output_filename)
        .expect(&format!("[ ERROR ] Could not create output file {}!", &output_filename));
    
    for line in &html_tokens {
        outfile.write_all(line.as_bytes())
               .expect("[ ERROR ] Could not write to output file!");
    }
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: tinymd [markdown file].md");
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => print_long_banner()
    }
}
