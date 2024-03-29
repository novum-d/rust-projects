# Router Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Frouter)](https://examples.yew.rs/router)

A blog all about yew.
The best way to figure out what this example is about is to just open it up.
It's mobile friendly too!

## Concepts

This example involves many different parts, here are just the Yew specific things:

- Uses [`yew-router`] to render and switch between multiple pages.

The example automatically adapts to the `--public-url` value passed to Trunk.
This allows it to be hosted on any path, not just at the root.
For example, our demo is hosted at [/router](https://examples.yew.rs/router).

This is achieved by adding `<base data-trunk-public-url />` to the [index.html](index.html) file.
Trunk rewrites this tag to contain the value passed to `--public-url` which can then be retrieved at runtime.
Take a look at [`Route`](src/main.rs) for the implementation.

## Improvements

- Use a special image component which shows a progress bar until the image is loaded.
- Scroll back to the top after switching route
- Run content generation in a dedicated web worker
- Use longer Markov chains to achieve more coherent results
- Make images deterministic (the same seed should produce the same images)
- Show posts by the author on their page
  (this is currently impossible because we need to find post seeds which in turn generate the author's seed)
- Show other posts at the end of a post ("continue reading")
- Home (`/`) should include links to the post list and the author introduction
- Detect sub-path from `--public-url` value passed to Trunk. See: thedodd/trunk#51

[`yew-router`]: https://docs.rs/yew-router/latest/yew_router/

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```

### Notes

Content generation can take up quite a bit of time in debug builds. If it is too slow, you should try running with the `release` profile.