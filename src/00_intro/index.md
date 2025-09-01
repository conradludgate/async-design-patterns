# Mastering Async

Understanding async design patterns and primitives

## Who am I

* Conrad Ludgate (he/him) - <https://conradludgate.com/>
* I do Rust systems programming at Neon (Databricks), we host Postgres with extra features - <https://neon.tech/>
* I really like async Rust.

## Where are the files?

<https://github.com/conradludgate/async-design-patterns>

## Where are the docs?

![](qrcode.gif)

<https://async-patterns.conrad.cafe/>

## What will this workshop cover

Async Rust is still evolving as a paradigm, but many common design patterns have emerged as useful.
Getting familiar with these patterns and tools will make it easier for you to develop async applications.

This workshop makes some assumptions, like that you will be using `tokio` and developing web-applications.
However, most of the ideas presented in this workshop will still map to other runtimes `monoio`, `glommio`, `embassy`,
and even map to other domains such as CLIs, desktop applications, databases, etc.
