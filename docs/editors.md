---
title: Editor support
---

## VS Code / Positron

Both VS Code and Positron have access to the Jarl extension via the [VS Marketplace](https://marketplace.visualstudio.com/items?itemName=EtienneBacher.jarl-vscode) and [Open VSX](https://open-vsx.org/extension/etiennebacher/jarl-vscode).

This extension provides code higlights and quick fixes:

* code highlights will underline pieces of code that violate any rule in your setup:

![](img/code_highlight.PNG)

* quick fixes lightbulb icons will appear when the cursor is next to a highlighted piece of code. Clicking this icon will allow you to apply the fix only for this piece of code:

![](img/code_quick_fix_1.PNG)

![](img/code_quick_fix_2.PNG)

![](img/code_quick_fix_3.PNG)


This extension provides few options integrated in VS Code or Positron.
One of them is "Assignment operator", that indicates which of `"="` or `"<-"` is preferred in the files parsed by Jarl.
This option can be set at the User or Workspace level by looking for "Jarl" in the IDE settings.

It is recommended to use [`jarl.toml`](config.md) if more configuration is needed. 

::: {.callout-tip}
The [Tombi extension](https://github.com/tombi-toml/tombi) is useful to have suggestions and autocompletion when editing `jarl.toml`.
:::
