# Fullstack Leptos-Cloudflare Template

## What is this:
Having trouble getting SSR Leptos to run as a Cloudflare worker?
Me too.

This Template is a close workaround:
- /leptos-wasm contains a CSR Leptos App with trunk and tailwindcss preconfigured.
- /axum-worker contains an axum worker

They are coupled so that you will only need to deploy one worker serving both your /api endpoints and your static files (including the actual wasm frontend-app).

### Advantages:
- You can use leptos and axum as worker together already today - making merging into a SSR Leptos Worker easy once that is more stable.
- Has Tailwindcss preconfigured; is otherwise unopinionated.
- Dead simple to use & blazingly fast

## How to setup:
- Make sure you have a Cloudflare Account.
- Clone the Repo
- That's it, you can start coding.

## How to use:
To **develop** with auto-reload, just run the dev.sh file.

To **deploy**, run
> trunk build --release

in /leptos-wasm and

> npx wrangler deploy

in /axum-worker.

## Disclaimer:
I dont take responsibility for shit. Cobbled this together in one afternoon. Use at your own risk.
