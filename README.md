# mdbook-content-collections

**Astro-style Content Collections for mdBook — zero JavaScript, pure Rust,
blazing fast.**

Bring the joy of
[Astro's Content Collections](https://docs.astro.build/en/guides/content-collections/)
to your mdBook projects. Automatically discover, parse, sort, and index all your
Markdown files with rich frontmatter — including previews, tags, collections,
drafts, and more — and get a single `content-collections.json` file in your
built book.

Perfect for blogs, documentation sites, personal wikis, or any mdBook project
that needs a structured content index for dynamic themes, search, filtering, or
RSS enhancements.

Example frontmatter:

```md
---
title: Unencrypted BTRFS Impermanence with Flakes
date: 2025-11-24
author: saylesss88
collection: blog
tags: ["nixos", "btrfs", "impermanence"]
draft: false
---

# Unencrypted BTRFS Impermanence with Flakes
```

And this is the generated `json`:

```json
{
  "entries": [
    {
      "author": "saylesss88",
      "collection": "blog",
      "date": "2025-11-24T00:00:00+00:00",
      "description": null,
      "draft": false,
      "path": "installation/unenc/unenc_impermanence.md",
      "preview_html": "<p>Figure 1: Impermanence Logo: Image of the Impermanence logo. Sourced from the</p><p><a href=\"https://github.com/nix-community/impermanence\">Impermanence repo</a></p><p>This guide is for an unencrypted setup, there are a few links at the end for\nencrypted setups. This guide follows the previous\n<a href=\"https://saylesss88.github.io/installation/unencrypted_setups.html\">minimal install guide</a>\nbut you should be able to adjust it carefully to meet your needs.</p>",
      "tags": [
        "nixos",
        "btrfs",
        "impermanence"
      ],
      "title": "Unencrypted BTRFS Impermanence with Flakes"
    },
## --snip--- ##
```

Your theme or external tools can then consume this index to build blog listings,
tag pages, feeds, or search indexes.​​

No extra build system. No Node.js. Just Rust + mdBook.

## Features

- Full frontmatter parsing (YAML)

- Smart date parsing (2025-11-24 or RFC3339)

- Fallback to file modification time

- Automatic HTML preview generation (first 3 paragraphs, cleaned)

- Intelligent boilerplate stripping (skips TOCs, metadata blocks)

- Supports collection, tags, draft, author, description

- Draft filtering (draft: true → excluded in production if you filter)

- Sorted newest first

- Outputs content-collections.json directly into your built book

- Works as a standalone mdBook preprocessor (just drop in book.toml)

## Installation

```bash
cargo install mdbook-content-collections
```

Version Check:

```bash
mdbook-content-collections --version
```

## Usage

Add to your `book.toml`:

```toml
[preprocessor.content-collections]
renderers = ["html"]
```

That’s it.

Run as usual:

```bash
mdbook build
# or
mdbook serve
```

Your content index will be available at:

```text
http://localhost:3000/content-collections.json
# or
https://your-site/content-collections.json
```

Outputs `content-collections.json` directly into your built book. (i.e.,
`src/content-collections.json`, and `book/content-collections.json`)

## Frontmatter Example

```yaml
---
title: My Awesome Post
date: 2025-11-24
author: saylesss88
description: A short summary (optional)
collection: blog
tags: ["nixos", "rust", "mdbook"]
draft: false
---
```

All fields are optional except `title` (falls back to filename).

### Use Cases

- Blog post listings

- Tag pages

- Author archives

- Search indexing (pair with mdbook-searcher or Lunr)

- Dynamic navigation

- RSS feed enhancement (pair with
  [mdbook-rss-feed](https://crates.io/crates/mdbook-rss-feed))

- Theme-powered collections (e.g. /blog, /notes, /til)

### Remove frontmatter from rendered HTML

See:
[mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip)

### Why This Exists

Astro made content fun again. mdBook is amazing for documentation and long-form
writing — but until now, it lacked a first-class way to query and list your
content. This crate closes that gap. No more hardcoding post lists. No more
fragile JS scraping. Just write Markdown with frontmatter, and get a powerful
content API for free.

### License

Apache-2.0

### Author

saylesss88 — proudly built with Rust and too much coffee.

Inspired by
[Astro Content Collections](https://docs.astro.build/en/guides/content-collections/)
· Powered by pulldown-cmark · Works great with mdbook-rss-feed and
mdbook-frontmatter-strip
