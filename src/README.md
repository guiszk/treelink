# treelink

Show all `href` links in a page, in the style of `tree`.

## about

Uses `reqwest` crate to send HTTP requests and `scraper` to parse.

This program prints the inner HTML and href values of all `a` elements.

## usage

`cargo run [--all] <url>`

Omit `-all` to exclude elements with html tags, such as `img` or `svg`.

Use `--all` to include all elements.
