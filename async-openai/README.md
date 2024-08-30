<div align="center">
  <a href="https://docs.rs/async-openai-types">
  </a>
</div>
<h1 align="center"> async-openai-types </h1>
<p align="center"> Async Rust library for OpenAI (Types Only)</p>
<div align="center">
    <a href="https://crates.io/crates/async-openai-types">
    <img src="https://img.shields.io/crates/v/async-openai-types.svg" />
    </a>
    <a href="https://docs.rs/async-openai-types">
    <img src="https://docs.rs/async-openai-types/badge.svg" />
    </a>
</div>
<div align="center">
</div>

## Overview

`async-openai-types` is based on `async-openai` crate,
and it removes all reqwest dependencies and contains only types.

Note that this is not 100% compatible with `async-openai` crate
since OpenAiError depends on reqwest::Error, which has been changed to Box<dyn std::error::Error>
and the variant name `ReqwestError` had been renamed to `RequestError`.
