<p align="center">
    <img width="1412" src="https://cdn.aleo.org/leo/banner.png">
</p>

<h1 align="center">The Leo Programming Language</h1>

<p align="center">
    <a href="https://circleci.com/gh/AleoHQ/leo"><img src="https://circleci.com/gh/AleoHQ/leo.svg?style=svg&circle-token=00960191919c40be0774e00ce8f7fa1fcaa20c00"></a>
    <a href="https://codecov.io/gh/AleoHQ/leo"><img src="https://codecov.io/gh/AleoHQ/leo/branch/testnet3/graph/badge.svg?token=S6MWO60SYL"/></a>
    <a href="https://discord.gg/5v2ynrw2ds"><img src="https://img.shields.io/discord/700454073459015690?logo=discord"/></a>
    <a href="https://allcontributors.org/"><img src="https://img.shields.io/github/all-contributors/AleoHQ/leo?color=ee8449&style=flat-square"/></a>
</p>

Leo is a functional, statically-typed programming language built for writing private applications.

## <a name='TableofContents'></a>Table of Contents

* [🍎 Overview](#-overview)
* [⚙️️ Build Guide](#-build-guide)
    * [🦀 Install Rust](#-install-rust)
    * [🐙 Build from Source Code](#-build-from-source-code)
* [🚀 Quick Start](#-quick-start)
* [🧰 Troubleshooting](#-troubleshooting)
* [📖 Documentation](#-documentation)
* [🤝 Contributing](#-contributing)
* [❤️ Contributors](#-contributors)
* [🛡️ License](#-license)


## 🍎 Overview

Welcome to the Leo programming language.

Leo provides a high-level language that abstracts low-level cryptographic concepts and makes it easy to 
integrate private applications into your stack. Leo compiles to circuits making zero-knowledge proofs practical.

The syntax of Leo is influenced by traditional programming languages like JavaScript, Scala, and Rust, with a strong emphasis on readability and ease-of-use.
Leo offers developers with tools to sanity check circuits including unit tests, integration tests, and console functions.

Leo is one part of a greater ecosystem for building private applications on [Aleo](https://aleo.org/). 
The language is currently in an alpha stage and is subject to breaking changes.

## ⚙️️ Build Guide 

### 🦀 Install Rust

We recommend installing Rust using [rustup](https://www.rustup.rs/). You can install `rustup` as follows:

- macOS or Linux:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows (64-bit):  
  
  Download the [Windows 64-bit executable](https://win.rustup.rs/x86_64) and follow the on-screen instructions.

- Windows (32-bit):  
  
  Download the [Windows 32-bit executable](https://win.rustup.rs/i686) and follow the on-screen instructions.

### 🐙 Build from Source Code

We recommend installing Leo by building from the source code as follows:

```bash
# Download the source code
git clone https://github.com/AleoHQ/leo
cd leo

# Install 'leo'
$ cargo install --path .
```

Now to use leo, in your terminal, run:
```bash
leo
```

## 🚀 Quick Start

Use the Leo CLI to create a new project

```bash
# create a new `hello-world` Leo project
leo new helloworld
cd helloworld

# build & setup & prove & verify
leo run
```

The `leo new` command creates a new Leo project with a given name.

The `leo run` command will compile the program into Aleo instructions and run it.

Congratulations! You've just run your first Leo program.

## 🧰 Troubleshooting
If you are having trouble installing and using Leo, please check out our [guide](docs/troubleshooting.md).

If the issue still persists, please [open an issue](https://github.com/AleoHQ/leo/issues/new/choose).

## 📖 Documentation

* [Hello World - Next Steps](https://developer.aleo.org/leo/hello)
* [Leo Language Documentation](https://developer.aleo.org/leo/language)
* [Leo ABNF Grammar](./docs/grammar/abnf-grammar.txt)
* [Homepage](https://developer.aleo.org/overview/)

## 🤝 Contributing
 
Please see our guidelines in the [developer documentation](./CONTRIBUTING.md)


## ❤️ Contributors
Thank you for helping make Leo better!  
[What do the emojis mean?🧐](https://allcontributors.org/docs/en/emoji-key)

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/d0cd"><img src="https://avatars.githubusercontent.com/u/23022326?v=4?s=100" width="100px;" alt="d0cd"/><br /><sub><b>d0cd</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=d0cd" title="Code">💻</a> <a href="#maintenance-d0cd" title="Maintenance">🚧</a> <a href="#question-d0cd" title="Answering Questions">💬</a> <a href="https://github.com/AleoHQ/leo/pulls?q=is%3Apr+reviewed-by%3Ad0cd" title="Reviewed Pull Requests">👀</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://leo-lang.org"><img src="https://avatars.githubusercontent.com/u/16715212?v=4?s=100" width="100px;" alt="Collin Chin"/><br /><sub><b>Collin Chin</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=collinc97" title="Code">💻</a> <a href="https://github.com/AleoHQ/leo/commits?author=collinc97" title="Documentation">📖</a> <a href="#maintenance-collinc97" title="Maintenance">🚧</a> <a href="https://github.com/AleoHQ/leo/pulls?q=is%3Apr+reviewed-by%3Acollinc97" title="Reviewed Pull Requests">👀</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/howardwu"><img src="https://avatars.githubusercontent.com/u/9260812?v=4?s=100" width="100px;" alt="Howard Wu"/><br /><sub><b>Howard Wu</b></sub></a><br /><a href="#ideas-howardwu" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-howardwu" title="Maintenance">🚧</a> <a href="#research-howardwu" title="Research">🔬</a> <a href="https://github.com/AleoHQ/leo/pulls?q=is%3Apr+reviewed-by%3Ahowardwu" title="Reviewed Pull Requests">👀</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://www.kestrel.edu/~coglio"><img src="https://avatars.githubusercontent.com/u/2409151?v=4?s=100" width="100px;" alt="Alessandro Coglio"/><br /><sub><b>Alessandro Coglio</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=acoglio" title="Documentation">📖</a> <a href="#maintenance-acoglio" title="Maintenance">🚧</a> <a href="#question-acoglio" title="Answering Questions">💬</a> <a href="https://github.com/AleoHQ/leo/pulls?q=is%3Apr+reviewed-by%3Aacoglio" title="Reviewed Pull Requests">👀</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://www.kestrel.edu/home/people/mccarthy/"><img src="https://avatars.githubusercontent.com/u/7607035?v=4?s=100" width="100px;" alt="Eric McCarthy"/><br /><sub><b>Eric McCarthy</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=bendyarm" title="Documentation">📖</a> <a href="#maintenance-bendyarm" title="Maintenance">🚧</a> <a href="#question-bendyarm" title="Answering Questions">💬</a> <a href="https://github.com/AleoHQ/leo/pulls?q=is%3Apr+reviewed-by%3Abendyarm" title="Reviewed Pull Requests">👀</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/raychu86"><img src="https://avatars.githubusercontent.com/u/14917648?v=4?s=100" width="100px;" alt="Raymond Chu"/><br /><sub><b>Raymond Chu</b></sub></a><br /><a href="#ideas-raychu86" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/AleoHQ/leo/commits?author=raychu86" title="Code">💻</a> <a href="#research-raychu86" title="Research">🔬</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ljedrz"><img src="https://avatars.githubusercontent.com/u/3750347?v=4?s=100" width="100px;" alt="ljedrz"/><br /><sub><b>ljedrz</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/issues?q=author%3Aljedrz" title="Bug reports">🐛</a> <a href="https://github.com/AleoHQ/leo/commits?author=ljedrz" title="Code">💻</a> <a href="#question-ljedrz" title="Answering Questions">💬</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Centril"><img src="https://avatars.githubusercontent.com/u/855702?v=4?s=100" width="100px;" alt="Mazdak Farrokhzad"/><br /><sub><b>Mazdak Farrokhzad</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=Centril" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://move-book.com"><img src="https://avatars.githubusercontent.com/u/8008055?v=4?s=100" width="100px;" alt="Damir Shamanaev"/><br /><sub><b>Damir Shamanaev</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=damirka" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/gluax"><img src="https://avatars.githubusercontent.com/u/16431709?v=4?s=100" width="100px;" alt="gluax"/><br /><sub><b>gluax</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=gluax" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/0rphon"><img src="https://avatars.githubusercontent.com/u/59403052?v=4?s=100" width="100px;" alt="0rphon"/><br /><sub><b>0rphon</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=0rphon" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Protryon"><img src="https://avatars.githubusercontent.com/u/8600837?v=4?s=100" width="100px;" alt="Max Bruce"/><br /><sub><b>Max Bruce</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=Protryon" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/isvforall"><img src="https://avatars.githubusercontent.com/u/706913?v=4?s=100" width="100px;" alt="Sergey Isaev"/><br /><sub><b>Sergey Isaev</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=isvforall" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/FranFiuba"><img src="https://avatars.githubusercontent.com/u/5733366?v=4?s=100" width="100px;" alt="Francisco Strambini"/><br /><sub><b>Francisco Strambini</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=FranFiuba" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://www.garillot.net/"><img src="https://avatars.githubusercontent.com/u/4142?v=4?s=100" width="100px;" alt="François Garillot"/><br /><sub><b>François Garillot</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=huitseeker" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://www.chenweikeng.com"><img src="https://avatars.githubusercontent.com/u/14937807?v=4?s=100" width="100px;" alt="Weikeng Chen"/><br /><sub><b>Weikeng Chen</b></sub></a><br /><a href="#research-weikengchen" title="Research">🔬</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/dev-sptg"><img src="https://avatars.githubusercontent.com/u/585251?v=4?s=100" width="100px;" alt="sptg"/><br /><sub><b>sptg</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/issues?q=author%3Adev-sptg" title="Bug reports">🐛</a> <a href="https://github.com/AleoHQ/leo/commits?author=dev-sptg" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://louiswt.github.io/"><img src="https://avatars.githubusercontent.com/u/22902565?v=4?s=100" width="100px;" alt="LouisWT"/><br /><sub><b>LouisWT</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=LouisWT" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/yuliyu123"><img src="https://avatars.githubusercontent.com/u/8566390?v=4?s=100" width="100px;" alt="yuliyu123"/><br /><sub><b>yuliyu123</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=yuliyu123" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://detailyang.github.io"><img src="https://avatars.githubusercontent.com/u/3370345?v=4?s=100" width="100px;" alt="detailyang"/><br /><sub><b>detailyang</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=detailyang" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Tom-OriginStorage"><img src="https://avatars.githubusercontent.com/u/103015469?v=4?s=100" width="100px;" alt="Tom-OriginStorage"/><br /><sub><b>Tom-OriginStorage</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=Tom-OriginStorage" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/omahs"><img src="https://avatars.githubusercontent.com/u/73983677?v=4?s=100" width="100px;" alt="omahs"/><br /><sub><b>omahs</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=omahs" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/HarukaMa"><img src="https://avatars.githubusercontent.com/u/861659?v=4?s=100" width="100px;" alt="Haruka"/><br /><sub><b>Haruka</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/issues?q=author%3AHarukaMa" title="Bug reports">🐛</a> <a href="https://github.com/AleoHQ/leo/commits?author=HarukaMa" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/swift-mx"><img src="https://avatars.githubusercontent.com/u/80231732?v=4?s=100" width="100px;" alt="swift-mx"/><br /><sub><b>swift-mx</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=swift-mx" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/all-contributors/all-contributors-bot"><img src="https://avatars3.githubusercontent.com/u/46843839?v=4?s=100" width="100px;" alt="allcontributors[bot]"/><br /><sub><b>allcontributors[bot]</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=allcontributors" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/actions"><img src="https://avatars.githubusercontent.com/u/65916846?v=4?s=100" width="100px;" alt="actions-user[bot]"/><br /><sub><b>actions-user[bot]</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=actions-user" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/features/security"><img src="https://avatars.githubusercontent.com/u/27347476?v=4?s=100" width="100px;" alt="dependabot[bot]"/><br /><sub><b>dependabot[bot]</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=dependabot" title="Documentation">📖</a></td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <td align="center" size="13px" colspan="7">
        <img src="https://raw.githubusercontent.com/all-contributors/all-contributors-cli/1b8533af435da9854653492b1327a23a4dbd0a10/assets/logo-small.svg">
          <a href="https://all-contributors.js.org/docs/en/bot/usage">Add your contributions</a>
        </img>
      </td>
    </tr>
  </tfoot>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

## 🛡️ License
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)
