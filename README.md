# Dev Dependencies

[Node JS](https://nodejs.org/en/download/)

[Rust](https://rustup.rs/)

[Docker](https://docs.docker.com/get-docker/)

Please use the latest stable versions of the above, and have them available in your [path](http://www.linfo.org/path_env_var.html#:~:text=PATH%20is%20an%20environmental%20variable,commands%20issued%20by%20a%20user.).

# Development
1. Run `./debug.sh`

# Deployment
1. Run `./build.sh` to compile a binary for linux.
1. Run `./deploy` to copy the file to the digital ocean vm.

---

Todo

- Step create. move instruction update and step create to the same file. one js file per page.

- When there are extra spaces between words in input (more than one, like "hi   there jesse"), it renders "hi there jesse" in the 'view' elements (like h1), but just like the input value when in the input. Ignored for now.

- makefile for not running the css build every time (depends on /templates/* files(both js and html)) (not priority, because i'm mostly doing front end right now, which will always require a rebuild. but what about the dev env? isn't that everything bundled? only change that when the css-modules change? (because that's when new generation needs to happen)). You're a recovering addict from things like this. You can learn them, yes, but not today.

What is this? Setting the environment to the user's environment? What if bash is not available
#!/usr/bin/env bash

<!-- Idea for a copy-paste runnable server:
hardcode database permissions, but not port. Conditional compile then?
curl 'howido'
PORT=80 ./howido

Maybe this is a part of building excitment? I mean, I think it's pretty cool.

The caveat? Understanding memory and I/O! Also, cross compiling.. if there's gonna be 'releases'
-->

What would a no-build javascript dependency system look like?

The idea is to avoid using webpack and to still use react (without JSX or typescript)

Components would be loaded from the web server, and cached. Libraries would be served from a CDN.

Tailwind would still have to be loaded via a build step - which is not so fun.

This would be a way to add interactivity to the client side in javascript. However, it would be mixing markup DOM with js that is pre-rendered which is bothersome. But that's how a *lot* of popular websites work!

Every javascript file would need hashing at build time, and every refrence would need to use that hash as it's file name. Just like the css. Which is what webpack does?