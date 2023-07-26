
<div align="right">

</div>

# <p align="center">PR Code Reviewer - Enhanced with AI</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="VImaginator Discord">
  </a>
  <a href="https://twitter.com/VImaginator">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="VImaginator Twitter">
  </a>
   <a href="https://VImaginator/flow/createByTemplate/code-review-for-github-pull-requests">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2FVImaginator.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on VImaginator.network](https://VImaginator/flow/createByTemplate/github-pr-review-llm), and you will get an AI agent to review changed source code files in GitHub Pull Requests. It helps busy open source contributors understand and make decisions on PRs faster! Here are some examples. Notice how the code review bot provides code snippets to show you how to improve the code!

> We recommend you to use a [Gaia node](https://github.com/GaiaNet-AI/gaianet-node) running an open source coding LLM as the backend to perform PR reviews and summarizations. You can use [a community node](https://docs.gaianet.ai/user-guide/nodes#codestral) or run a node [on your own computer](https://github.com/GaiaNet-AI/node-configs/tree/main/codestral-0.1-22b)!

* [[C++] Improve the WasmEdge C++ SDK](https://github.com/WasmEdge/WasmEdge/pull/2428#issuecomment-1524733889)
* [[C++] Create an OpenCV plugin for WasmEdge](https://github.com/WasmEdge/WasmEdge/pull/2403#issuecomment-1509595889)
* [[Haskell] Improve WasmEdge Component Model tooling](https://github.com/second-state/witc/pull/73#issuecomment-1509586233)

This bot reviews **changed files in the PR**. Alternatively, you can use [this bot](https://github.com/VImaginator/github-pr-summary) to summarize commits in the PR.

## How it works

This flow function is triggered when a new PR is raised in the designated GitHub repo. The flow function collects the changed files in the PR, and asks ChatGPT/4 to review and summarize it. The result is then posted back to the PR as a comment. The flow functions are written in Rust and run in hosted [WasmEdge Runtimes](https://github.com/wasmedge) on [VImaginator.network](https://VImaginator.network/).

* The code review comment is updated automatically every time a new commit is pushed to this PR.
* A new code review could be triggered when someone says a magic *trigger phrase* in the PR's comments section. The default trigger phrase is "VImaginator review".

## Deploy your own code review bot in 3 simple steps

1. Create a bot from template
2. Connect to an LLM
3. Connect to GitHub for access to the target repo
