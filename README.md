<h2 align="center">:earth_africa: ryvex :earth_africa:</h2>

<p align="center">
    <a href="https://github.com/Flokkq/ryvex/actions">
        <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Flokkq/smd/main.yaml?style=for-the-badge&logo=githubactions&color=DD7878&logoColor=D9E0EE&labelColor=302D41">
    </a>
    <a href="https://github.com/Flokkq/ryvex?tab=readme-ov-file#currently-supported-targets">
        <img alt="Targets" src="https://img.shields.io/badge/Targets-3-DD7878?style=for-the-badge&logo=rust&color=CA9EE6&logoColor=D9E0EE&labelColor=302D41"/>
    </a>
	<a href="https://github.com/flokkq/nixOS/stargazers">
		<img alt="Stargazers" src="https://img.shields.io/github/stars/flokkq/nixOS?style=for-the-badge&logo=starship&color=C9CBFF&logoColor=D9E0EE&labelColor=302D41">
    </a>
</p>

ryvex is a

- **cross-platform**
- **dependency free**
- **terminal based**
- **vim-like**

text editor written in Rust.

## Goal

The goal of ryvex is to **support as many targets as possible**. Whether it’s a smart fridge, a web browser or a custom operating system—if it can execute code, ryvex should run there.

## Currently Supported Targets

> [!Note]
> Once v0.1.0 is released, all supported targets will be available via AUR, Nix, Homebrew, Chocolatey and GitHub Releases.

- macOS
- Linux
- Windows

Web browsers and WASI are planned as the next targets.

## Contributing

> [!Important]
> ryvex is under active development and is not yet stable. Adding support for a new will require ongoing maintenance, as future features might demand additional platform-specific code.

If your preferred platform is not listed, you can add support by implementing the required traits for filesystem access and rendering in [ryvex-target](./ryvex-target/src/impl/). Any target is welcome; there are absolutely no restrictions on what you can add.
