# mdbook-content-collections

**Astro-style Content Collections for mdBook, zero JavaScript, pure Rust,
blazing fast.**

Bring the utility of
[Astro's Content Collections](https://docs.astro.build/en/guides/content-collections/)
to your mdBook projects. Automatically discover, parse, sort, and index all your
Markdown files with rich frontmatter, including previews, tags, collections,
drafts, and more, and get a single `content-collections.json` file in your built
book.

Perfect for blogs, documentation sites, personal wikis, or any mdBook project
that needs a structured content index for dynamic themes, search, filtering, or
RSS enhancements.

Example frontmatter:

```md
---
title: Unencrypted BTRFS Impermanence with Flakes
date: 2025-11-27
author: saylesss88
collection: blog
tags: ["nixos", "btrfs", "impermanence"]
draft: false
---
```

And this is the generated `json` output, showing a few chapters for context:

```json
{
  "entries": [
    {
      "author": "saylesss88",
      "collection": "blog",
      "date": "2025-11-27T00:00:00+00:00",
      "description": null,
      "draft": false,
      "path": "Nix_Pull_Requests_11.md",
      "preview_html": "<p><img src=\"images/gruv16.png\" alt=\"gruv16\" /></p><p><strong>Pull requests</strong> communicate changes...</p>",
      "tags": [
        "nixos",
        "nixpkgs"
      ],
      "title": "Nix Pull Requests"
    },
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
```

_Example truncated for brevity_

Your theme or external tools can then consume this index to build blog listings,
tag pages, feeds, or search indexes.​​

No extra build system. No Node.js. Just Rust + mdBook.

---

## Features

- Full frontmatter parsing (YAML)

- Smart date parsing (`2025-11-24` or RFC3339)

- Fallback to file modification time

- Automatic HTML preview generation (first 3 paragraphs, cleaned)

- Intelligent boilerplate stripping (skips TOCs, metadata blocks)

- Supports collection, tags, draft, author, description

- Draft filtering (`draft: true` → excluded in production if you filter)

- Sorted newest first

- Outputs `content-collections.json` directly into your built book

- Works as a standalone mdBook preprocessor (just drop in `book.toml`)

---

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

---

### Use Cases

- Blog post listings

- Tag pages

- Author archives

- Search indexing (pair with mdbook-searcher or Lunr)

- Dynamic navigation

- RSS feed enhancement (pair with
  [mdbook-rss-feed](https://crates.io/crates/mdbook-rss-feed))

- Theme-powered collections (e.g. /blog, /notes, /til)

---

### Remove frontmatter from rendered HTML

See:
[mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip)

---

### Why This Exists

Astro made content fun again. mdBook is amazing for documentation and long-form
writing. But until now, it lacked a first-class way to query and list your
content. This crate closes that gap. No more hardcoding post lists. No more
fragile JS scraping. Just write Markdown with frontmatter, and get a powerful
content API for free.

---

### Example: “Latest posts” preview on your landing page

If you want a zero‑fetch setup, pair this crate with the companion preprocessor
[mdbook-content-loader](https://crates.io/crates/mdbook-content-loader). That
preprocessor reads the generated `content-collections.json` at build time and
injects it into every page as `window.CONTENT_COLLECTIONS`, so your theme JS can
render widgets without any extra HTTP requests.

---

<details>
<summary> ✔️ Click to Expand Example standalone Usage </summary>

We can take advantage of this generated `content-collections.json` to show a
rendered preview of your last modified content. This is just an example and not
production ready but pretty cool nonetheless.

Extend your `theme/index.hbs` scroll down near the bottom and add this block

Find this code block near line 270 in `index.hbs`:

```js
      {{#if next}}
          <a rel="next prefetch" href="{{ path_to_root }}{{next.link}}" class="nav-chapters next" title="Next chapter" aria-label="Next chapter" aria-keyshortcuts="Right">
              {{#if (eq text_direction "rtl")}}
              {{fa "solid" "angle-left"}}
              {{else}}
              {{fa "solid" "angle-right"}}
              {{/if}}
          </a>
      {{/if}}

//    ### <div id="content-collections-list" class="content-collections-list">
```

And place the following code right below the above code block.

```js
  <div id="content-collections-list" class="content-collections-list">
    <!-- Populated by mdbook-content-collections -->
  </div>

   <script>
    (function () {
      var indexUrl = window.location.origin + window.location.pathname.replace(/\/[^\/]*$/, '') + '/content-collections.json';

      if (window.location.protocol === 'file:') {
        indexUrl = 'content-collections.json';
      }

      fetch(indexUrl)
        .then(function (res) {
          if (!res.ok) throw new Error('Failed to load content-collections.json');
          return res.json();
        })
        .then(function (data) {
          if (!data || !Array.isArray(data.entries)) return;

          var container = document.getElementById('content-collections-list');
          if (!container) return;

          var entries = data.entries
            .filter(function (e) {
              return !e.draft && (!e.collection || e.collection === 'blog');
            })
            .slice(0, 10);

          if (entries.length === 0) {
            container.textContent = 'No posts yet.';
            return;
          }

          var list = document.createElement('ul');
          list.className = 'content-collections-items';

          entries.forEach(function (e) {
            var li = document.createElement('li');
            li.className = 'content-collections-item';

            var link = document.createElement('a');
            var htmlPath = e.path.replace(/\.md(?:own|arkdown)?$/i, '.html');
            link.href = htmlPath;
            link.textContent = e.title || e.path;

            var meta = document.createElement('div');
            meta.className = 'content-collections-meta';

            if (e.date) {
              var d = new Date(e.date);
              meta.textContent = d.toISOString().slice(0, 10);
            }

            var preview = document.createElement('div');
            preview.className = 'content-collections-preview';
            preview.innerHTML = e.preview_html || '';

            li.appendChild(link);
            if (meta.textContent) li.appendChild(meta);
            li.appendChild(preview);
            list.appendChild(li);
          });

          container.innerHTML = '';
          container.appendChild(list);
        })
        .catch(function (err) {
          console.warn('mdbook-content-collections:', err);
        });
    })();
  </script>
```

Now, your books landing page will have an extension above the prev-next chapter
buttons showing a rendered list/overview of your last modified content. Taken
from the generated `content-collections.json`:

![content-collections](https://raw.githubusercontent.com/saylesss88/mdbook-content-collections/main/assets/content-collections.png)

</details>

---

## Project built using `mdbook-content-collections`

[`mdbook-kanagawa-theme`](https://crates.io/crates/mdbook-kanagawa-theme) shows
how to use `content-collections.json` to drive a dynamic landing page: it reads
the generated collections, builds cards for posts/notes based on frontmatter,
and injects them into `index.md`. All of the wiring (JS, HTML, CSS) lives in the
preprocessor and theme, so you can get a “Latest posts” layout without editing
`theme/index.hbs` yourself.

![mdbook-kanagawa](https://raw.githubusercontent.com/saylesss88/mdbook-content-collections/main/assets/swappy-20251205-072621.cleaned.png)

---

### License

[Apache License 2.0](https://github.com/saylesss88/mdbook-content-collections/blob/main/LICENSE)

Inspired by
[Astro Content Collections](https://docs.astro.build/en/guides/content-collections/)
· Powered by [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)
· Works great with mdbook-rss-feed, mdbook-frontmatter-strip, and
mdbook-content-loader.
