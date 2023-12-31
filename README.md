# HTOML

**HTOML**, also known as HyperText Obvious Language, is a transpiler written in Rust that transpile TOML that follows a specific protocol to HTML. 

**This project is in development and also not production-ready.**

## HTOML Protocol

**HTOML Protocol** is that specific protocol in question. It limits how TOML can be translated to HTML and aims to provide a smooth learning curve to HTML developer and a modern way to edit your HTML page. 

It should noticed that HTOML and HTOML protocol is two separated things. **HTOML** is a transpiler that transpile TOML file that follows HTOML Protocol to valid HTML. **HTOML Protocol** is just a protocol. It is *neither* a transpiler nor a compiler.

## Getting started

1. Create a TOML file with extension `.toml`.

2. Edit this file with your favourite editor.

3. Entering the editor, type `html = "html"`. This key-value pair is for declaring the HTML version, like `<!DOCTYPE html>`. It will be a declaration of HTOML file.

4. (TODO)

## Currently support

1. Some HTML elements

    - Most of the non-empty/void elements
    - Simple empty/void elements:
        - br
        - hr
    - Simple head elements:
        - title
    - Element `a` with `href` attribute 

2. HTML version and `lang` attribute for `body` element

## LICENSE

HTOML (also include HTOML protocol) is created under the MIT License. Please see `LICENSE` file for more inforamtion.

