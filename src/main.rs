use regex::Regex;

fn main() {
    // check cli args
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("usage: cargo run [--all] <url>");
        std::process::exit(0);
    }

    let mut arg = "".to_string();
    let mut printall = false;
    for i in args {
        if i == "--all" {
            printall = true;
        } else {
            arg = i;
        }
    }

    println!("{}", arg);
    
    // make request
    let response = reqwest::blocking::get(arg).unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&response);
    // parse links
    let linkselector = scraper::Selector::parse("a").unwrap();

    // ignore tags
    let tagregex = Regex::new(r"<[^>]+>").unwrap();
    let mut last = false;

    for (i, a) in document.select(&linkselector).enumerate() {
        if i == document.select(&linkselector).count() - 1 {
            last = true;
        }

        let val = a.inner_html();

        // in case href is None
        let mut href = a.value().attr("href");
        if let None = href {
            href = Some("");
        }
        let href = href.unwrap();

        // print "└──" instead of "├──" for last item
        if val != "" {
            if printall || ! tagregex.is_match(val.as_str()) {
                let prefix = if last { "└──" } else { "├──" };
                println!("{} {} -> {}", prefix, val, href);
            }
        }
    }
}
