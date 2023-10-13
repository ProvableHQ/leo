<p align="center">
    <img width="1412" src="https://cdn.aleo.org/leo/banner.png">
</p>

<h1 align="center">The Leo Programming Language</h1>

<p align="center">
    <a href="https://circleci.com/gh/AleoHQ/leo"><img src="https://circleci.com/gh/AleoHQ/leo.svg?style=svg&circle-token=00960191919c40be0774e00ce8f7fa1fcaa20c00"></a>
    <a href="https://codecov.io/gh/AleoHQ/leo"><img src="https://codecov.io/gh/AleoHQ/leo/branch/testnet3/graph/badge.svg?token=S6MWO60SYL"/></a>
    <a href="https://discord.gg/5v2ynrw2ds"><img src="https://img.shields.io/discord/700454073459015690?logo=discord"/></a>
    <a href="https://GitHub.com/AleoHQ/leo"><img src="https://img.shields.io/badge/contributors-29-ee8449"/></a>
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
* [Leo ABNF Grammar](https://github.com/AleoHQ/grammars/blob/master/leo.abnf)
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
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/aharshbe"><img src="https://avatars.githubusercontent.com/u/17191728?v=4?s=100" width="100px;" alt="aharshbe"/><br /><sub><b>aharshbe</b></sub></a><br /><a href="https://github.com/aharshbe/test_leo_app" title="Tutorials">✅</a><a href="https://github.com/AleoHQ/leo/issues?q=author%3Aaharshbe" title="Bug reports">🐛</a> <a href="#question-aharshbe" title="Answering Questions">💬</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Centril"><img src="https://avatars.githubusercontent.com/u/855702?v=4?s=100" width="100px;" alt="Mazdak Farrokhzad"/><br /><sub><b>Mazdak Farrokhzad</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=Centril" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://move-book.com"><img src="https://avatars.githubusercontent.com/u/8008055?v=4?s=100" width="100px;" alt="Damir Shamanaev"/><br /><sub><b>Damir Shamanaev</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=damirka" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/gluax"><img src="https://avatars.githubusercontent.com/u/16431709?v=4?s=100" width="100px;" alt="gluax"/><br /><sub><b>gluax</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=gluax" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/0rphon"><img src="https://avatars.githubusercontent.com/u/59403052?v=4?s=100" width="100px;" alt="0rphon"/><br /><sub><b>0rphon</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=0rphon" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Protryon"><img src="https://avatars.githubusercontent.com/u/8600837?v=4?s=100" width="100px;" alt="Max Bruce"/><br /><sub><b>Max Bruce</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=Protryon" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/isvforall"><img src="https://avatars.githubusercontent.com/u/706913?v=4?s=100" width="100px;" alt="Sergey Isaev"/><br /><sub><b>Sergey Isaev</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=isvforall" title="Code">💻</a></td>
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
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/FranFiuba"><img src="https://avatars.githubusercontent.com/u/5733366?v=4?s=100" width="100px;" alt="Francisco Strambini"/><br /><sub><b>Francisco Strambini</b></sub></a><br /><a href="https://github.com/AleoHQ/leo/commits?author=FranFiuba" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/dangush"><img src="https://avatars.githubusercontent.com/u/39884512?v=4?s=100" width="100px;" alt="Daniel Gushchyan"/><br /><sub><b>Daniel Gushchyan</b></sub></a><br /><a href="https://github.com/dangush/aleo-lottery" title="Tutorials">✅</a></td>
    <td align="center" valign="top" width="14.28%"><a href="https://github.com/r4keta"><img src="https://avatars.githubusercontent.com/u/78550627?v=4?s=100" width="100px;" alt="r4keta"/><br /><sub><b>r4keta</b></sub></a><br /><a href="https://github.com/r4keta/ihar-tic-tac-toe" title="Tutorial">✅</a></td> 
    <td align="center" valign="top" width="14.28%"><a href="https://github.com/liolikus"><img src="https://avatars.githubusercontent.com/u/85246338?v=4?s=100" width="100px;" alt="liolikus"/><br /><sub><b>liolikus</b></sub></a><br /><a href="https://github.com/liolikus/quiz_token_with_username" title="Content">🖋</a></td>
    </tr>
    <tr>  
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/evgeny-garanin"><img src="https://avatars.githubusercontent.com/u/44749897?v=4?s=100" width="100px;" alt="evgeny-garanin"/><br /><sub><b>Evgeny Garanin</b></sub></a><br /><a href="https://github.com/evgeny-garanin/aleoapp" title=“Tutorial>✅</a></td>
         <td align="center" valign="top" width="14.28%"><a href="https://github.com/NickoMenty"><img src="https://avatars.githubusercontent.com/u/52633108?s=80&v=4?s=100" width="100px;" alt="NickoMenty"/><br /><sub><b>NickoMenty</b></sub></a><br /><a href="https://github.com/NickoMenty/tictacapp" title=“Tutorial>✅</a></td>
        <td align="center" valign="top" width="14.28%"><a href="https://github.com/eug33ne"><img src="https://avatars.githubusercontent.com/u/146975479?s=80&v=4?s=100" width="100px;" alt="eug33ne"/><br /><sub><b>eug33ne</b></sub></a><br /><a href="https://github.com/eug33ne/eugenettt" title=“Tutorial>✅</a></td>
    <td align="center" valign="top" width="14.28%"><a href="https://github.com/Nininiao"><img src="https://avatars.githubusercontent.com/u/75372952?s=80&v=4?s=100" width="100px;" alt="Nininiao"/><br /><sub><b>Nininiao</b></sub></a><br /><a href="https://github.com/Nininiao/tictactoe-aleo" title=“Tutorial>✅</a></td>
        <td align="center" valign="top" width="14.28%"><a href="https://github.com/CTurE1"><img src="https://avatars.githubusercontent.com/u/93711669?s=80&v=4?s=100" width="100px;" alt="CTurE1"/><br /><sub><b>CTurE1</b></sub></a><br /><a href="https://github.com/CTurE1/leo_first" title=“Tutorial>✅</a></td>
        <td align="center" valign="top" width="14.28%"><a href="https://github.com/colliseum2006"><img src="https://avatars.githubusercontent.com/u/26433623?s=80&v=4?s=100" width="100px;" alt="colliseum2006"/><br /><sub><b>colliseum2006</b></sub></a><br /><a href="https://github.com/colliseum2006/Aleo-TicTacToe-Leo" title=“Tutorial>✅</a></td>
        <td align="center" valign="top" width="14.28%"><a href="https://github.com/boaaa"><img src="https://avatars.githubusercontent.com/u/18523852?s=80&u=dafb625808ba6ebe266ffb090c32294ba5cd1978&v=4?s=100" width="100px;" alt="boaaa"/><br /><sub><b>boaaa</b></sub></a><br /><a href="https://github.com/boaaa/leo-tictactoe" title=“Tutorial>✅</a></td>
    </tr>
      <tr>
        <td align="center" valign="top" width="14.28%"><a href="https://github.com/HausenUA"><img src="https://avatars.githubusercontent.com/u/107180551?s=80&u=767d5b3fa32499e1a9bd199195464b23a0a2a5ff&v=4?s=100" width="100px;" alt="HausenUA"/><br /><sub><b>HausenUA</b></sub></a><br /><a href="https://github.com/HausenUA/lotteryAleo" title=“Tutorial>✅</a></td>
        <td align="center" valign="top" width="14.28%"><a href="https://github.com/TerrenceTepezano"><img src="https://avatars.githubusercontent.com/u/90051964?s=80&v=4?s=100" width="100px;" alt="TerrenceTepezano"/><br /><sub><b>TerrenceTepezano</b></sub></a><br /><a href="https://github.com/TerrenceTepezano/leo-example-lottery" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/Zabka0x94"><img src="https://avatars.githubusercontent.com/u/118641707?s=80&v=4?s=100" width="100px;" alt="Zabka0x94"/><br /><sub><b>Zabka0x94</b></sub></a><br /><a href="https://github.com/Zabka0x94/TarasLottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/DarronHanly1"><img src="https://avatars.githubusercontent.com/u/90051711?s=80&v=4?s=100" width="100px;" alt="DarronHanly1"/><br /><sub><b>DarronHanly1</b></sub></a><br /><a href="https://github.com/DarronHanly1/tictactoe-leo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/penglang"><img src="https://avatars.githubusercontent.com/u/90052701?s=80&u=005c2163e9ce71c4b4c5057b9633387bb7b07d3a&v=4?s=100" width="100px;" alt="penglang"/><br /><sub><b>
FengXiaoYong</b></sub></a><br /><a href="https://github.com/penglang/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/KassieSteinman"><img src="https://avatars.githubusercontent.com/u/90052202?s=80&v=4?s=100" width="100px;" alt="KassieSteinman"/><br /><sub><b>
KassieSteinman</b></sub></a><br /><a href="https://github.com/KassieSteinman/example-lottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/MaishaAzim"><img src="https://avatars.githubusercontent.com/u/90052288?s=80&v=4?s=100" width="100px;" alt="MaishaAzim"/><br /><sub><b>
MaishaAzim</b></sub></a><br /><a href="https://github.com/MaishaAzim/lottery" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Moria-Bright"><img src="https://avatars.githubusercontent.com/u/147031372?s=80&u=c8ee842648f3c7beeae0f6096d7b0727c3726e6d&v=4?s=100" width="100px;" alt="Moria-Bright"/><br /><sub><b>
Moria Bright</b></sub></a><br /><a href="https://github.com/Moria-Bright/Leo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Bradshow"><img src="https://avatars.githubusercontent.com/u/147033772?s=80&u=80056bd706b952de2871e4715515b50f92b997fd&v=4?s=100" width="100px;" alt="Bradshow"/><br /><sub><b>
Bradshow</b></sub></a><br /><a href="https://github.com/Bradshow/lottery-Leo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/SilvaHoffarth"><img src="https://avatars.githubusercontent.com/u/90052391?s=80&v=4?s=100" width="100px;" alt="SilvaHoffarth"/><br /><sub><b>
SilvaHoffarth</b></sub></a><br /><a href="https://github.com/SilvaHoffarth/example-tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Elaine1015"><img src="https://avatars.githubusercontent.com/u/147033872?s=80&u=a5830cb86421eb9fa013c1dc2c2c1bc459bf2410&v=4?s=100" width="100px;" alt="Elaine1015"/><br /><sub><b>
Elaine1015</b></sub></a><br /><a href="https://github.com/Elaine1015/Lottery" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/vasylbelyi"><img src="https://avatars.githubusercontent.com/u/101014717?s=80&v=4?s=100" width="100px;" alt="vasylbelyi"/><br /><sub><b>
vasylbelyi</b></sub></a><br /><a href="https://github.com/vasylbelyi/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/EgorMajj"><img src="https://avatars.githubusercontent.com/u/91486022?s=80&u=ab2183b3999a1773e16d19a342b3f0333fb79aef&v=4?s=100" width="100px;" alt="EgorMajj"/><br /><sub><b>
EgorMajj</b></sub></a><br /><a href="https://github.com/EgorMajj/egormajj-Aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/RNS23"><img src="https://avatars.githubusercontent.com/u/93403404?s=80&v=4?s=100" width="100px;" alt="RNS23"/><br /><sub><b>
RNS23</b></sub></a><br /><a href="https://github.com/RNS23/aleo-project" title=“Tutorial>✅</a></td>
      </tr>
       <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/VoinaOleksandr"><img src="https://avatars.githubusercontent.com/u/123416145?s=80&v=4?s=100" width="100px;" alt="VoinaOleksandr"/><br /><sub><b>
VoinaOleksandr</b></sub></a><br /><a href="https://github.com/Moria-Bright/Leo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/alexprimak58"><img src="https://avatars.githubusercontent.com/u/78500984?s=80&u=8d86ccc0909f74a99beaa91659f72ea1fc210425&v=4?s=100" width="100px;" alt="alexprimak58"/><br /><sub><b>
alexprimak58</b></sub></a><br /><a href="https://github.com/alexprimak58/aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Asimous22"><img src="https://avatars.githubusercontent.com/u/123984389?s=80&u=a48284738bc8e7650e8f01c586bb21614f167a4a&v=4?s=100" width="100px;" alt="Asimous22"/><br /><sub><b>
Asimous22</b></sub></a><br /><a href="https://github.com/Asimous22/AleooL1" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Marik0023"><img src="https://avatars.githubusercontent.com/u/70592085?s=80&v=4?s=100" width="100px;" alt="Marik0023"/><br /><sub><b>
Marik0023</b></sub></a><br /><a href="https://github.com/Marik0023/Aleo" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/JanSchluter"><img src="https://avatars.githubusercontent.com/u/90052550?s=80&v=4?s=100" width="100px;" alt="JanSchluter"/><br /><sub><b>
JanSchluter</b></sub></a><br /><a href="https://github.com/JanSchluter/leo-token" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/AminaPerrigan"><img src="https://avatars.githubusercontent.com/u/90052692?s=80&v=4?s=100" width="100px;" alt="AminaPerrigan"/><br /><sub><b>
AminaPerrigan</b></sub></a><br /><a href="https://github.com/AminaPerrigan/aleo-lottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Utah8O"><img src="https://avatars.githubusercontent.com/u/147143937?s=80&v=4?s=100" width="100px;" alt="Utah8O"/><br /><sub><b>
Utah8O</b></sub></a><br /><a href="https://github.com/Utah8O/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
       <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/ApoloniaResseguie"><img src="https://avatars.githubusercontent.com/u/90052780?s=80&v=4ApoloniaResseguie?s=100" width="100px;" alt="ApoloniaResseguie"/><br /><sub><b>
ApoloniaResseguie</b></sub></a><br /><a href="https://github.com/ApoloniaResseguie/aleo-example-token" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/NobukoCausley"><img src="https://avatars.githubusercontent.com/u/90052877?s=80&v=4?s=100" width="100px;" alt="NobukoCausley"/><br /><sub><b>
NobukoCausley</b></sub></a><br /><a href="https://github.com/NobukoCausley/example-project-tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/ololo70"><img src="https://avatars.githubusercontent.com/u/123416859?s=80&v=4?s=100" width="100px;" alt="ololo70"/><br /><sub><b>
ololo70</b></sub></a><br /><a href="https://github.com/ololo70/lottery.aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/evangelion4215"><img src="https://avatars.githubusercontent.com/u/147157455?s=80&u=8676ba262e019b3c49758a78d0a22cb207c119f1&v=4?s=100" width="100px;" alt="evangelion4215"/><br /><sub><b>
evangelion4215</b></sub></a><br /><a href="https://github.com/evangelion4215/aleorepository" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/boodovskiy"><img src="https://avatars.githubusercontent.com/u/15303736?s=80&v=4?s=100" width="100px;" alt="boodovskiy"/><br /><sub><b>
boodovskiy</b></sub></a><br /><a href="https://github.com/boodovskiy/leo-app-alexbud" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/BULVER777"><img src="https://avatars.githubusercontent.com/u/78557232?s=80&v=4?s=100" width="100px;" alt="BULVER777"/><br /><sub><b>
BULVER777</b></sub></a><br /><a href="https://github.com/BULVER777/Leo_Developer" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Slashxdd"><img src="https://avatars.githubusercontent.com/u/32466372?s=80&u=e8cf936790566cdb518e4dce14a2824666aac3a6&v=4?s=100" width="100px;" alt="Slashxdd"/><br /><sub><b>
Kyrylo Budovskyi</b></sub></a><br /><a href="https://github.com/Slashxdd/leo-example" title=“Tutorial>✅</a></td>
             <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/sayber1717"><img src="https://avatars.githubusercontent.com/u/107244636?s=80&v=4?s=100" width="100px;" alt="sayber1717"/><br /><sub><b>
sayber1717</b></sub></a><br /><a href="https://github.com/sayber1717/aleo-first" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/BudiSwy"><img src="https://avatars.githubusercontent.com/u/147084162?s=80&u=b30985bab45cd7379abe08555c2d3a0e81df4b28&v=4?s=100" width="100px;" alt="BudiSwy
"/><br /><sub><b>
BudiSwy
</b></sub></a><br /><a href="https://github.com/BudiSwy/BudiSwyLottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/romacll"><img src="https://avatars.githubusercontent.com/u/138483707?s=80&v=4?s=100" width="100px;" alt="romacll"/><br /><sub><b>
romacll</b></sub></a><br /><a href="https://github.com/romacll/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/habaroff18203"><img src="https://avatars.githubusercontent.com/u/37939150?s=80&v=4?s=100" width="100px;" alt="habaroff18203"/><br /><sub><b>
habaroff18203</b></sub></a><br /><a href="https://github.com/habaroff18203/Tic-tac-toe-Aleo" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/LennyPro6"><img src="https://avatars.githubusercontent.com/u/119447436?s=80&v=4?s=100" width="100px;" alt="LennyPro6"/><br /><sub><b>
LennyPro6</b></sub></a><br /><a href="https://github.com/LennyPro6/AleoTictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/n0d4"><img src="https://avatars.githubusercontent.com/u/127042589?s=80&v=4?s=100" width="100px;" alt="n0d4"/><br /><sub><b>
n0d4</b></sub></a><br /><a href="https://github.com/n0d4/tictactoe1" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/grossbel12"><img src="https://avatars.githubusercontent.com/u/86624298?s=80&u=03e4eb8a1f5200f0ea8393ad94f5350fb35c3d0c&v=4?s=100" width="100px;" alt="grossbel12"/><br /><sub><b>
grossbel12</b></sub></a><br /><a href="https://github.com/grossbel12/Test_privat_Aleo" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Orliha"><img src="https://avatars.githubusercontent.com/u/89811794?s=80&v=4?s=100" width="100px;" alt="Orliha"/><br /><sub><b>
Orliha</b></sub></a><br /><a href="https://github.com/Orliha/battleshiponaleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/darijn"><img src="https://avatars.githubusercontent.com/u/77969911?s=80&u=b22be029487be6034ccc2349351f1da442916581&v=4?s=100" width="100px;" alt="darjin"/><br /><sub><b>
darjin
</b></sub></a><br /><a href="https://github.com/darijn/aleoappbyme" title=“Content>🖋</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/romacll"><img src="https://avatars.githubusercontent.com/u/138483707?s=80&v=4?s=100" width="100px;" alt="romacll"/><br /><sub><b>
romacll</b></sub></a><br /><a href="https://github.com/romacll/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/aleoweb123"><img src="https://avatars.githubusercontent.com/u/123852645?s=80&v=4?s=100" width="100px;" alt="aleoweb123"/><br /><sub><b>
aleoweb123</b></sub></a><br /><a href="https://github.com/aleoweb123/tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/arosboro"><img src="https://avatars.githubusercontent.com/u/2224595?s=80&v=4?s=100" width="100px;" alt="arosboro"/><br /><sub><b>
Andrew Rosborough</b></sub></a><br /><a href="https://github.com/arosboro/newsletter" title=“Content>🖋</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/R-Demon"><img src="https://avatars.githubusercontent.com/u/74899343?s=80&v=4?s=100" width="100px;" alt="R-Demon"/><br /><sub><b>
R-Demon</b></sub></a><br /><a href="https://github.com/R-Demon/Leo-test" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/sryykov"><img src="https://avatars.githubusercontent.com/u/144407047?s=80&v=4?s=100" width="100px;" alt="sryykov"/><br /><sub><b>
sryykov</b></sub></a><br /><a href=" https://github.com/sryykov/lottery" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/himera0482"><img src="https://avatars.githubusercontent.com/u/147270825?s=80&v=4?s=100" width="100px;" alt="himera0482"/><br /><sub><b>
himera0482</b></sub></a><br /><a href="https://github.com/himera0482/lotteryHimera" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/encipher88"><img src="https://avatars.githubusercontent.com/u/36136421?s=80&u=75315d2db3508972320ecfdb2a39698ceac5aabc&v=4?s=100" width="100px;" alt="encipher88"/><br /><sub><b>
encipher88
</b></sub></a><br /><a href="https://github.com/encipher88/aleoapplottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Likaenigma"><img src="https://avatars.githubusercontent.com/u/82119648?s=80&v=4?s=100" width="100px;" alt="Likaenigma"/><br /><sub><b>
Likaenigma</b></sub></a><br /><a href="https://github.com/Likaenigma/Aleo_tictaktoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/bartosian"><img src="https://avatars.githubusercontent.com/u/20209819?s=80&u=f02ed67ada96f4f128a48a437cdb9064e4d978a1&v=4?s=100" width="100px;" alt="bartosian"/><br /><sub><b>
bartosian</b></sub></a><br /><a href="https://github.com/bartosian/tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/bendenizrecep"><img src="https://avatars.githubusercontent.com/u/61727501?s=80&u=96b0aa75990afc2feceb87dd6e9de44984e7a42d&v=4?s=100" width="100px;" alt="bendenizrecep"/><br /><sub><b>
Recep Deniz</b></sub></a><br /><a href="https://github.com/bendenizrecep/Aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Saimon87"><img src="https://avatars.githubusercontent.com/u/97917099?s=80&v=4?s=100" width="100px;" alt="Saimon87"/><br /><sub><b>
Saimon87</b></sub></a><br /><a href="https://github.com/Saimon87/lottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/BannyNo"><img src="https://avatars.githubusercontent.com/u/105598886?s=80&u=6bb32e2dec2bfff0e81a97da2932d3bde4761b2d&v=4?s=100" width="100px;" alt="BannyNo"/><br /><sub><b>
Big Ixela</b></sub></a><br /><a href="https://github.com/BannyNo/ttk" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Mistmorn0"><img src="https://avatars.githubusercontent.com/u/132354087?s=80&u=949b312989a7c7214da6eda067a955d73051abe4&v=4?s=100" width="100px;" alt="Mistmorn0"/><br /><sub><b>
Denys Riabets </b></sub></a><br /><a href="https://github.com/Mistmorn0/tic-tac-toe-aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/chipqp"><img src="https://avatars.githubusercontent.com/u/147347780?s=80&u=ce7d8206896790577a4806b50a5b410df0171f55&v=4?s=100" width="100px;" alt="chipqp"/><br /><sub><b>
Dmytro Groma
</b></sub></a><br /><a href="https://github.com/chipqp/chipqplotteryforAleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/VolodymyrRudoi"><img src="https://avatars.githubusercontent.com/u/147347334?s=80&v=4?s=100" width="100px;" alt="VolodymyrRudoi"/><br /><sub><b>
Volodymyr Rudoi</b></sub></a><br /><a href="https://github.com/VolodymyrRudoi/RudoiLeoTicTacToe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/petrofalatyuk"><img src="https://avatars.githubusercontent.com/u/147347836?s=80&v=4?s=100" width="100px;" alt="petrofalatyuk"/><br /><sub><b>
Petro Falatiuk
</b></sub></a><br /><a href="https://github.com/petrofalatyuk/Aleo-lottery" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/eleven-pixel"><img src="https://avatars.githubusercontent.com/u/68178877?s=80&u=8520dc290911b4a613180bb5fa9c46f0cde769b4&v=4?s=100" width="100px;" alt="eleven-pixel "/><br /><sub><b>
ElsaChill</b></sub></a><br /><a href="https://github.com/eleven-pixel/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/gsulaberidze"><img src="https://avatars.githubusercontent.com/u/98606008?s=80&v=4?s=100" width="100px;" alt="gsulaberidze"/><br /><sub><b>
gsulaberidze</b></sub></a><br /><a href="https://github.com/gsulaberidze/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/kegvorn"><img src="https://avatars.githubusercontent.com/u/98895367?s=80&u=8eb56f5a9ca694c0b659a47eaceda18a2d075f04&v=4?s=100" width="100px;" alt="kegvorn"/><br /><sub><b>
kegvorn</b></sub></a><br /><a href="https://github.com/kegvorn/aleo_kegvorn" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Porcoss"><img src="https://avatars.githubusercontent.com/u/116500991?s=80&v=4?s=100" width="100px;" alt="totoro_me"/><br /><sub><b>
totoro_me</b></sub></a><br /><a href="https://github.com/Porcoss/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/timchinskiyalex"><img src="https://avatars.githubusercontent.com/u/69203707?s=80&v=4?s=100" width="100px;" alt="timchinskiyalex"/><br /><sub><b>
timchinskiyalex
</b></sub></a><br /><a href="https://github.com/timchinskiyalex/aleo_test_token" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/DimaSpys"><img src="https://avatars.githubusercontent.com/u/102924787?s=80&v=4?s=100" width="100px;" alt="DimaSpys"/><br /><sub><b>
DimaSpys</b></sub></a><br /><a href="https://github.com/DimaSpys/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/DimBirch"><img src="https://avatars.githubusercontent.com/u/99015099?s=80&u=72ec17d4ca64b433bb725247b311ecfb4795f2f3&v=4?s=100" width="100px;" alt="dimbirch"/><br /><sub><b>
dimbirch
</b></sub></a><br /><a href="https://github.com/DimBirch/tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/YuraPySHIT"><img src="https://avatars.githubusercontent.com/u/147433702?s=80&v=4?s=100" width="100px;" alt="YuraPySHIT "/><br /><sub><b>
YuraPySHIT</b></sub></a><br /><a href="https://github.com/YuraPySHIT/ChokavoLottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/annabirch"><img src="https://avatars.githubusercontent.com/u/116267741?s=80&v=4?s=100" width="100px;" alt="annabirch"/><br /><sub><b>
annabirch</b></sub></a><br /><a href="https://github.com/annabirch/lottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/baxzban"><img src="https://avatars.githubusercontent.com/u/34492472?s=80&v=4?s=100" width="100px;" alt="baxzban"/><br /><sub><b>
baxzban</b></sub></a><br /><a href="https://github.com/baxzban/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/nnewera3"><img src="https://avatars.githubusercontent.com/u/101011598?s=80&u=14eb50f6ffd51968e44ec9d8273c6aac7a0912fc&v=4?s=100" width="100px;" alt="nnewera3"/><br /><sub><b>
nnewera3</b></sub></a><br /><a href="https://github.com/nnewera3/newera3" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/LabLinens"><img src="https://avatars.githubusercontent.com/u/92609032?s=80&u=317c54f560c9d49f99bcf6b2826f58d2b68245c6&v=4?s=100" width="100px;" alt="LabLinens"/><br /><sub><b>
LabLinens
</b></sub></a><br /><a href="https://github.com/LabLinens/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/drimartist"><img src="https://avatars.githubusercontent.com/u/147176569?s=80&u=267b3d70952a55ad7f1e6194e084c8781dc0c3f5&v=4?s=100" width="100px;" alt="drimartist"/><br /><sub><b>
drimartist</b></sub></a><br /><a href="https://github.com/drimartist/tic-tac-toe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/savarach"><img src="https://avatars.githubusercontent.com/u/92996312?s=80&v=4?s=100" width="100px;" alt="savarach"/><br /><sub><b>
savarach
</b></sub></a><br /><a href="https://github.com/savarach/tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/padjfromdota"><img src="https://avatars.githubusercontent.com/u/147413251?s=80&v=4?s=100" width="100px;" alt="padjfromdota "/><br /><sub><b>
padjfromdota</b></sub></a><br /><a href="https://github.com/padjfromdota/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/gglorymen"><img src="https://avatars.githubusercontent.com/u/38043626?s=80&u=0cb8966e52f12395f6eeccb4c183633f7607efb3&v=4?s=100" width="100px;" alt="gglorymen"/><br /><sub><b>
gglorymen</b></sub></a><br /><a href="https://github.com/iLRuban/staraleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/KrisMorisBoris"><img src="https://avatars.githubusercontent.com/u/147434887?s=80&u=80beb8bbd23c869ea2e15eddbc45826c908965a0&v=4?s=100" width="100px;" alt="KrisMorisBoris"/><br /><sub><b>
KrisMorisBoris</b></sub></a><br /><a href="https://github.com/KrisMorisBoris/Leoapp2" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/WebDuster"><img src="https://avatars.githubusercontent.com/u/147457876?s=80&v=4?s=100" width="100px;" alt="WebDuster"/><br /><sub><b>
WebDuster</b></sub></a><br /><a href="https://github.com/WebDuster/TicTacToe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Tasham2008"><img src="https://avatars.githubusercontent.com/u/88756708?s=80&u=5f8d877a473c61435bf6c1b26ea0e5f6d45bb378&v=4?s=100" width="100px;" alt="Tasham2008"/><br /><sub><b>
Tasham2008
</b></sub></a><br /><a href="https://github.com/Tasham2008/Aleo_tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/760AnaPY"><img src="https://avatars.githubusercontent.com/u/53938257?s=80&v=4?s=100" width="100px;" alt="760AnaPY"/><br /><sub><b>
760AnaPY</b></sub></a><br /><a href="https://github.com/760AnaPY/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/imshelest"><img src="https://avatars.githubusercontent.com/u/147422014?s=80&v=4?s=100" width="100px;" alt="imshelest"/><br /><sub><b>
imshelest
</b></sub></a><br /><a href="https://github.com/imshelest/leo1" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/mirmalnir"><img src="https://avatars.githubusercontent.com/u/73130193?s=80&v=4?s=100" width="100px;" alt="mirmalnir"/><br /><sub><b>
mirmalnir</b></sub></a><br /><a href="https://github.com/mirmalnir/tictatoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/AnatoliMP"><img src="https://avatars.githubusercontent.com/u/95178926?s=80&v=4?s=100" width="100px;" alt="AnatoliMP"/><br /><sub><b>
AnatoliMP</b></sub></a><br /><a href="https://github.com/AnatoliMP/AleoOneLove" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/ihortym"><img src="https://avatars.githubusercontent.com/u/101022021?s=80&v=4?s=100" width="100px;" alt="ihortym"/><br /><sub><b>
ihortym</b></sub></a><br /><a href="https://github.com/ihortym/Aleo.git" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Vplmrchk"><img src="https://avatars.githubusercontent.com/u/147513906?s=80&u=a7133949fa694f8e7dcbfc5ec182bac7e3db9d49&v=4?s=100" width="100px;" alt="Vplmrchk"/><br /><sub><b>
Vplmrchk</b></sub></a><br /><a href="https://github.com/Vplmrchk/lotteryV_plmrchk" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/anrd04"><img src="https://avatars.githubusercontent.com/u/96128115?s=80&v=4?s=100" width="100px;" alt="anrd04"/><br /><sub><b>
anrd04
</b></sub></a><br /><a href="https://github.com/anrd04/tictak" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Gonruk"><img src="https://avatars.githubusercontent.com/u/124696038?s=80&v=4?s=100" width="100px;" alt="Gonruk"/><br /><sub><b>
Gonruk</b></sub></a><br /><a href="https://github.com/Gonruk/Firsttictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/ur4ix"><img src="https://avatars.githubusercontent.com/u/100270373?s=80&v=4?s=100" width="100px;" alt="ur4ix"/><br /><sub><b>
ur4ix
</b></sub></a><br /><a href="https://github.com/ur4ix/Aleo_Tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/AllininanQ"><img src="https://avatars.githubusercontent.com/u/147525847?s=80&v=4?s=100" width="100px;" alt="AllininanQ"/><br /><sub><b>
AllininanQ</b></sub></a><br /><a href="https://github.com/AllininanQ/leo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Juliaaa26"><img src="https://avatars.githubusercontent.com/u/130294051?s=80&v=4?s=100" width="100px;" alt="Juliaaa26"/><br /><sub><b>
Juliaaa26</b></sub></a><br /><a href="https://github.com/Juliaaa26/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Hacker-web-Vi"><img src="https://avatars.githubusercontent.com/u/80550154?s=80&u=7b71cbd476b43e06e83a7a7470a774d26c6d7cd1&v=4?s=100" width="100px;" alt="Hacker-web-Vi"/><br /><sub><b>
Hacker-web-Vi</b></sub></a><br /><a href="https://github.com/Hacker-web-Vi/leo-developer_toolkit" title=“Tutorial>✅</a></td>
      </tr>
       <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Mickey1245"><img src="https://avatars.githubusercontent.com/u/122784690?s=80&u=67a7ee12d2de04031d187b0af9361c16776276aa&v=4?s=100" width="100px;" alt="Mickey1245"/><br /><sub><b>
Mickey1245</b></sub></a><br /><a href="https://github.com/Mickey1245/MickeyALEO" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/anastesee"><img src="https://avatars.githubusercontent.com/u/97472175?s=80&u=54eae625d094a13c9a7eaa1e3385e9db2c570832&v=4?s=100" width="100px;" alt="anastese"/><br /><sub><b>
anastese
</b></sub></a><br /><a href="https://github.com/anastesee/leo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/NastyaTR97"><img src="https://avatars.githubusercontent.com/u/147534568?s=80&u=e2c4cf66ba2de9d52a047a1f01a98dc52cc81a72&v=4?s=100" width="100px;" alt="NastyaTR97"/><br /><sub><b>
NastyaTR97</b></sub></a><br /><a href="https://github.com/NastyaTR97/tictactoeTrofimovaA" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/andriypaska"><img src="https://avatars.githubusercontent.com/u/130220653?s=80&u=9c9e72a1278d9fe8b6943181abde3b0e01e3a1a7&v=4?s=100" width="100px;" alt="andriypaska"/><br /><sub><b>
andriypaska
</b></sub></a><br /><a href="https://github.com/andriypaska/tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/dendistar"><img src="https://avatars.githubusercontent.com/u/138825246?s=80&u=f5313c3e3b802a46a3f0cd2f1d92266ab7a459dd&v=4?s=100" width="100px;" alt="dendistar"/><br /><sub><b>
dendistar</b></sub></a><br /><a href="https://github.com/dendistar/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/kartaviy223"><img src="https://avatars.githubusercontent.com/u/147543231?s=80&v=4?s=100" width="100px;" alt="kartaviy223"/><br /><sub><b>
kartaviy223</b></sub></a><br /><a href="https://github.com/kartaviy223/aleo123/tree/main/Aleoapp" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/BluePEz"><img src="https://avatars.githubusercontent.com/u/147533370?s=80&v=4?s=100" width="100px;" alt="BluePEz"/><br /><sub><b>
BluePEz</b></sub></a><br /><a href="https://github.com/BluePEz/aleo-tictactoe" title=“Tutorial>✅</a></td>
      </tr>
        <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Ihorika2"><img src="https://avatars.githubusercontent.com/u/147540567?s=80&u=f4de57b4b3e6552fd715e85376552be3e22c4177&v=4?s=100" width="100px;" alt="Ihorika2"/><br /><sub><b>
Ihorika2</b></sub></a><br /><a href="https://github.com/Ihorika2/aleo1" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/taraspaska"><img src="https://avatars.githubusercontent.com/u/130307768?s=80&v=4?s=100" width="100px;" alt="taraspaska"/><br /><sub><b>
taraspaska
</b></sub></a><br /><a href="https://github.com/taraspaska/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Ragnaros12q"><img src="https://avatars.githubusercontent.com/u/147474896?s=80&u=815c1097456eacd4d0e2eb4aa9c21747f7b9f518&v=4?s=100" width="100px;" alt="Ragnaros12q"/><br /><sub><b>
Ragnaros12q</b></sub></a><br /><a href="https://github.com/Ragnaros12q/testnet-aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/StasFreeman"><img src="https://avatars.githubusercontent.com/u/88969589?s=80&v=4?s=100" width="100px;" alt="StasFreeman"/><br /><sub><b>
StasFreeman
</b></sub></a><br /><a href="https://github.com/StasFreeman/tictactoeStasFreeman" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/McTrick"><img src="https://avatars.githubusercontent.com/u/100270374?s=80&v=4?s=100" width="100px;" alt="McTrick"/><br /><sub><b>
McTrick</b></sub></a><br /><a href="https://github.com/McTrick/tictactoeTr1ck" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Dimaleron"><img src="https://avatars.githubusercontent.com/u/147550161?s=80&v=4?s=100" width="100px;" alt="Dimaleron"/><br /><sub><b>
Dimaleron</b></sub></a><br /><a href="https://github.com/Dimaleron/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Boruto11dw"><img src="https://avatars.githubusercontent.com/u/120184733?s=80&v=4?s=100" width="100px;" alt="Boruto11dw"/><br /><sub><b>
Boruto11dw</b></sub></a><br /><a href="https://github.com/Merlin-clasnuy/Boruto__.git" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/NOne790"><img src="https://avatars.githubusercontent.com/u/147545650?s=80&v=4?s=100" width="100px;" alt="NOne790"/><br /><sub><b>
NOne790</b></sub></a><br /><a href="https://github.com/NOne790/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Golldirr"><img src="https://avatars.githubusercontent.com/u/147552484?s=80&v=4?s=100" width="100px;" alt="Golldirr"/><br /><sub><b>
Golldirr
</b></sub></a><br /><a href="https://github.com/Golldirr/AleoG.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/dmytriievp"><img src="https://avatars.githubusercontent.com/u/141562373?s=80&v=4?s=100" width="100px;" alt="dmytriievp"/><br /><sub><b>
dmytriievp</b></sub></a><br /><a href="https://github.com/dmytriievp/Aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/InfernoCyber55"><img src="https://avatars.githubusercontent.com/u/147475467?s=80&v=4?s=100" width="100px;" alt="InfernoCyber55"/><br /><sub><b>
InfernoCyber55
</b></sub></a><br /><a href="https://github.com/InfernoCyber55/leolanguage" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/dexxeed"><img src="https://avatars.githubusercontent.com/u/90214222?s=80&v=4?s=100" width="100px;" alt="dexxeed"/><br /><sub><b>
dexxeed</b></sub></a><br /><a href="https://github.com/dexxeed/leoba.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/kumarman1"><img src="https://avatars.githubusercontent.com/u/147553980?s=80&u=2728032bbe99b024a5251485369a583aee5b7b8a&v=4?s=100" width="100px;" alt="kumarman1
"/><br /><sub><b>
kumarman1
</b></sub></a><br /><a href="https://github.com/kumarman1/kumarman.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/nika040"><img src="https://avatars.githubusercontent.com/u/95068350?s=80&v=4?s=100" width="100px;" alt="nika040"/><br /><sub><b>
nika040</b></sub></a><br /><a href="https://github.com/nika040/aleo1.git" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Collins44444444444444"><img src="https://avatars.githubusercontent.com/u/147554050?s=80&v=4?s=100" width="100px;" alt="Collins44444444444444"/><br /><sub><b>
Collins44444444444444</b></sub></a><br /><a href="https://github.com/Collins44444444444444/Collins" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/aavegotch"><img src="https://avatars.githubusercontent.com/u/147549770?s=80&u=0dad7648d64ad0199dcfaf4b83ab578ea94b6295&v=4?s=100" width="100px;" alt="aavegotch"/><br /><sub><b>
aavegotch
</b></sub></a><br /><a href="https://github.com/aavegotch/al-aav" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/ssvitlyk"><img src="https://avatars.githubusercontent.com/u/60655698?s=80&u=92087fbbda5739ad9fb3ebf19c78fea1573b7cf7&v=4?s=100" width="100px;" alt="ssvitlyk"/><br /><sub><b>
Sergiy Svitlyk</b></sub></a><br /><a href="https://github.com/ssvitlyk/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Mariia077"><img src="https://avatars.githubusercontent.com/u/93621050?s=80&u=0e86339f7d355f7bbe4ab7d67b8e5e04074c3819&v=4?s=100" width="100px;" alt="Mariia077"/><br /><sub><b>
Mariia077
</b></sub></a><br /><a href="https://github.com/Mariia077/tictactoe.git" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/svitlykihor"><img src="https://avatars.githubusercontent.com/u/118134393?s=80&u=903f10ba76ed251986a92ab908de563a4d77a6ee&v=4?s=100" width="100px;" alt="svitlykihor"/><br /><sub><b>
svitlykihor</b></sub></a><br /><a href="https://github.com/svitlykihor/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/dmytrohayov"><img src="https://avatars.githubusercontent.com/u/110791993?s=80&v=4?s=100" width="100px;" alt="dmytrohayov
"/><br /><sub><b>
Dmytro Haiov
</b></sub></a><br /><a href="https://github.com/dmytrohayov/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Annnnnnnnnnna"><img src="https://avatars.githubusercontent.com/u/40041762?s=80&v=4?s=100" width="100px;" alt="Annnnnnnnnnna"/><br /><sub><b>
Annnnnnnnnnna</b></sub></a><br /><a href="https://github.com/Annnnnnnnnnna/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/turchmanovich101"><img src="https://avatars.githubusercontent.com/u/68894538?s=80&v=4?s=100" width="100px;" alt="turchmanovich101"/><br /><sub><b>
turchmanovich101</b></sub></a><br /><a href="https://github.com/turchmanovich101/tictactoe2" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Zasmin12ve"><img src="https://avatars.githubusercontent.com/u/147555748?s=80&v=4?s=100" width="100px;" alt="Zasmin12ve"/><br /><sub><b>
Zasmin12ve
</b></sub></a><br /><a href="https://github.com/Zasmin12ve/Zasmin" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/timfaden"><img src="https://avatars.githubusercontent.com/u/94048988?s=80&u=9d5aee80da43319dfed966b32af5515a1d19bba6&v=4?s=100" width="100px;" alt="timfaden"/><br /><sub><b>timfaden</b></sub></a><br /><a href="https://github.com/timfaden/4Aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/MerlinKlasnuy"><img src="https://avatars.githubusercontent.com/u/147555707?s=80&v=4?s=100" width="100px;" alt="MerlinKlasnuy"/><br /><sub><b>
MerlinKlasnuy
</b></sub></a><br /><a href="https://github.com/MerlinKlasnuy/Merlin_Klasnuy" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/Erikprimerov"><img src="https://avatars.githubusercontent.com/u/82612075?s=80&u=de44b74d829e703e6b43627a0c61078a5eceaa1d&v=4?s=100" width="100px;" alt="erikprimerov"/><br /><sub><b>
erikprimerov</b></sub></a><br /><a href="https://github.com/Erikprimerov/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Andreewko"><img src="https://avatars.githubusercontent.com/u/128628158?s=80&u=580be033987939689565e11621b87003e565c56b&v=4?s=100" width="100px;" alt="Andreewko"/><br /><sub><b>Andreewko</b></sub></a><br /><a href="https://github.com/Andreewko/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/dxungngh"><img src="https://avatars.githubusercontent.com/u/6395634?s=80&v=4?s=100" width="100px;" alt="dxungngh"/><br /><sub><b>
Daniel Nguyen</b></sub></a><br /><a href="https://github.com/dxungngh/aleosample" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/igorstrong"><img src="https://avatars.githubusercontent.com/u/128728865?s=80&u=0d1cdb3d8ad159489d96814de771e8e13b090d63&v=4?s=100" width="100px;" alt="igorstrong"/><br /><sub><b>
igorstrong</b></sub></a><br /><a href="https://github.com/igorstrong/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/kramarmakarena"><img src="https://avatars.githubusercontent.com/u/107809808?s=80&u=fb9c3590aed168fd2de8317f81ecc76d6576d05e&v=4?s=100" width="100px;" alt="kramarmakarena"/><br /><sub><b>
Kramar Maxim
</b></sub></a><br /><a href="https://github.com/kramarmakarena/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/boichka"><img src="https://avatars.githubusercontent.com/u/109759533?s=80&u=59589e2c3b9088f651164d6d2664cfbec2f6d63f&v=4?s=100" width="100px;" alt="boichka"/><br /><sub><b>Marina Boyko</b></sub></a><br /><a href="https://github.com/boichka/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/YaakovHuang"><img src="https://avatars.githubusercontent.com/u/9527803?s=80&v=4?s=100" width="100px;" alt="YaakovHuang"/><br /><sub><b>
YaakovHuang
</b></sub></a><br /><a href="https://github.com/YaakovHuang/tictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/viktoria3715"><img src="https://avatars.githubusercontent.com/u/147585653?s=80&v=4?s=100" width="100px;" alt="viktoria3715"/><br /><sub><b>
viktoria3715</b></sub></a><br /><a href="https://github.com/viktoria3715/Leoapp" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Hello-World99bit"><img src="https://avatars.githubusercontent.com/u/122752681?s=80&v=4?s=100" width="100px;" alt="Hello-World99bit"/><br /><sub><b>Hello-World99bit</b></sub></a><br /><a href="https://github.com/Hello-World99bit/aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Alan-Zarevskij"><img src="https://avatars.githubusercontent.com/u/147600040?s=80&v=4?s=100" width="100px;" alt="Alan-Zarevskij"/><br /><sub><b>
Alan-Zarevskij</b></sub></a><br /><a href="https://github.com/Alan-Zarevskij/aleo-guide" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Huliko"><img src="https://avatars.githubusercontent.com/u/147601130?s=80&v=4?s=100" width="100px;" alt="Huliko"/><br /><sub><b>
Huliko</b></sub></a><br /><a href="https://github.com/Huliko/tutorial-aleo-game" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/tommy1qwerty"><img src="https://avatars.githubusercontent.com/u/147488401?s=80&v=4?s=100" width="100px;" alt="tommy1qwerty"/><br /><sub><b>
tommy1qwerty</b></sub></a><br /><a href="https://github.com/tommy1qwerty/Aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/sueinz"><img src="https://avatars.githubusercontent.com/u/75493321?s=80&v=4?s=100" width="100px;" alt="sueinz"/><br /><sub><b>sueinz</b></sub></a><br /><a href="https://github.com/sueinz/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Julia-path"><img src="https://avatars.githubusercontent.com/u/147602421?s=80&v=4?s=100" width="100px;" alt="Julia-path"/><br /><sub><b>
Julia-path
</b></sub></a><br /><a href="https://github.com/Julia-path/aleo-amb-tut" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/web3tyan"><img src="https://avatars.githubusercontent.com/u/73800674?s=80&u=c4d42f981b16acf70b786b5d400fb30be80e69fa&v=4?s=100" width="100px;" alt="web3tyan"/><br /><sub><b>
Diana Shershun</b></sub></a><br /><a href="https://github.com/web3tyan/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/mcnk020"><img src="https://avatars.githubusercontent.com/u/75666384?s=80&v=4?s=100" width="100px;" alt="mcnk020"/><br /><sub><b>mcnk020</b></sub></a><br /><a href="https://github.com/mcnk020/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Edgar0515"><img src="https://avatars.githubusercontent.com/u/82619131?s=80&v=4?s=100" width="100px;" alt="Edgar0515"/><br /><sub><b>
Edgar0515</b></sub></a><br /><a href="https://github.com/Edgar0515/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Ju1issa"><img src="https://avatars.githubusercontent.com/u/115104650?s=80&u=11a40da1c64bbdca41ac08934b132c45943e917f&v=4?s=100" width="100px;" alt="Ju1issa"/><br /><sub><b>
Ju1issa</b></sub></a><br /><a href="https://github.com/Ju1issa/Aleo-contibution-1" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/MGavrilo"><img src="https://avatars.githubusercontent.com/u/63003898?s=80&v=4?s=100" width="100px;" alt="MGavrilo"/><br /><sub><b>
MGavrilo</b></sub></a><br /><a href="https://github.com/MGavrilo/aleo_token.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/YujiROO1"><img src="https://avatars.githubusercontent.com/u/140186161?s=80&u=0311f4a1fed71c9e83c1b491903999160ca570fb&v=4?s=100" width="100px;" alt="YujiROO1"/><br /><sub><b>YujiROO1</b></sub></a><br /><a href="https://github.com/YujiROO1/firsttryLEOroyhansen" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/yuriyMiller"><img src="https://avatars.githubusercontent.com/u/20724500?s=80&v=4?s=100" width="100px;" alt="yuriyMiller"/><br /><sub><b>
yuriyMiller</b></sub></a><br /><a href="https://github.com/yuriyMiller/contribution_AToken" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/tetianapvlnk"><img src="https://avatars.githubusercontent.com/u/110791850?s=80&v=4?s=100" width="100px;" alt="tetianapvlnk"/><br /><sub><b>
Tetiana Pavlenko</b></sub></a><br /><a href="https://github.com/tetianapvlnk/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/MGavrilo"><img src="https://avatars.githubusercontent.com/u/63003898?s=80&v=4?s=100" width="100px;" alt="MGavrilo"/><br /><sub><b>MGavrilo</b></sub></a><br /><a href="https://github.com/MGavrilo/aleo_token" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/vizimnokh"><img src="https://avatars.githubusercontent.com/u/87230321?v=4?s=100" width="100px;" alt="vizimnokh"/><br /><sub><b>
vizimnokh</b></sub></a><br /><a href="https://github.com/vizimnokh/vi.app" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/oleksvit"><img src="https://avatars.githubusercontent.com/u/107810228?s=80&u=96f8b2c67161a457889e89ddaafff86d95d1e899&v=4?s=100" width="100px;" alt="oleksvit"/><br /><sub><b>
Oleksii Svitlyk</b></sub></a><br /><a href="https://github.com/oleksvit/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/t3s1"><img src="https://avatars.githubusercontent.com/u/68332636?s=80&u=a31e34ba9ebaf8cc46969dc02123dfbdf35238c2&v=4?s=100" width="100px;" alt="t3s1"/><br /><sub><b>
t3s1</b></sub></a><br /><a href="https://github.com/t3s1/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/BloodBand"><img src="https://avatars.githubusercontent.com/u/103063619?s=80&v=4?s=100" width="100px;" alt="BloodBand"/><br /><sub><b>BloodBand</b></sub></a><br /><a href="https://github.com/BloodBand/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/thereisnspoon"><img src="https://avatars.githubusercontent.com/u/74349032?v=4?s=100" width="100px;" alt="thereisnspoon"/><br /><sub><b>
thereisnspoon</b></sub></a><br /><a href="https://github.com/thereisnspoon/MyAleotictactoe" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/InfernoCyber55"><img src="https://avatars.githubusercontent.com/u/147475467?s=80&v=4?s=100" width="100px;" alt="InfernoCyber55"/><br /><sub><b>
InfernoCyber55</b></sub></a><br /><a href="https://github.com/InfernoCyber55/leolanguage" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Pikkorio1"><img src="https://avatars.githubusercontent.com/u/147637636?s=80&v=4?s=100" width="100px;" alt="Pikkorio1"/><br /><sub><b>Pikkorio1</b></sub></a><br /><a href="https://github.com/Pikkorio1/pikkorio" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/quertc"><img src="https://avatars.githubusercontent.com/u/48246993?s=80&u=6ef48157b7fcfac27beda4c346ec44d2fc71053d&v=4?s=100" width="100px;" alt="quertc"/><br /><sub><b>
quertc</b></sub></a><br /><a href="https://github.com/quertc/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Yuriihrk"><img src="https://avatars.githubusercontent.com/u/147640009?s=80&v=4?s=100" width="100px;" alt="Yuriihrk"/><br /><sub><b>
Yuriihrk</b></sub></a><br /><a href="https://github.com/Yuriihrk/YuriiHrkLottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/stsefa"><img src="https://avatars.githubusercontent.com/u/147640614?s=80&v=4?s=100" width="100px;" alt="stsefa"/><br /><sub><b>
stsefa</b></sub></a><br /><a href="https://github.com/stsefa/Lola13" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/alanharper"><img src="https://avatars.githubusercontent.com/u/1077736?s=80&u=83e0401d0d992dda6c7b6f491b1e87e68b9606b2&v=4?s=100" width="100px;" alt="alanharper"/><br /><sub><b>Alan Harper</b></sub></a><br /><a href="https://github.com/alanharper/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/imanbtc"><img src="https://avatars.githubusercontent.com/u/35306074?s=80&u=e9af87e9ff55a793649fa4c2640c8dc5a4ec05a8&v=4?s=100" width="100px;" alt="imanbtc"/><br /><sub><b>
imanbtc</b></sub></a><br /><a href="https://github.com/imanbtc/tictactoe.git" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/Oleksandr7744"><img src="https://avatars.githubusercontent.com/u/80430485?s=80&v=4?s=100" width="100px;" alt="Oleksandr7744"/><br /><sub><b>
Oleksandr7744</b></sub></a><br /><a href="https://github.com/Oleksandr7744/tictactoe777" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/MarikJudo"><img src="https://avatars.githubusercontent.com/u/89316361?s=80&v=4?s=100" width="100px;" alt="MarikJudo"/><br /><sub><b>MarikJudo</b></sub></a><br /><a href="https://github.com/MarikJudo/ticktacktoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Piermanenta"><img src="https://avatars.githubusercontent.com/u/147656191?s=80&v=4?s=100" width="100px;" alt="Piermanenta"/><br /><sub><b>
Piermanenta</b></sub></a><br /><a href="https://github.com/Piermanenta/LEo" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Karoliniio"><img src="https://avatars.githubusercontent.com/u/147644152?s=80&v=4?s=100" width="100px;" alt="Karoliniio"/><br /><sub><b>
Karoliniio</b></sub></a><br /><a href="https://github.com/Karoliniio/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/aixen1009"><img src="https://avatars.githubusercontent.com/u/70536452?s=80&u=3ed3b2bac8db9dd2b289176b08a9cd0b72b0d30b&v=4?s=100" width="100px;" alt="aixen1009"/><br /><sub><b>
Olga Svitlyk</b></sub></a><br /><a href="https://github.com/aixen1009/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/khanaya9845"><img src="https://avatars.githubusercontent.com/u/74767726?s=80&u=f92a94b69a04fd8724e7fbb6ee8f07b66302b571&v=4?s=100" width="100px;" alt="khanaya9845"/><br /><sub><b>khanaya9845</b></sub></a><br /><a href="https://github.com/khanaya9845/tictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/OlgaBurd"><img src="https://avatars.githubusercontent.com/u/147664595?s=80&v=4?s=100" width="100px;" alt="OlgaBurd"/><br /><sub><b>
OlgaBurd</b></sub></a><br /><a href="https://github.com/OlgaBurd/olgatictactoealeo" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/YaakovHunag920515"><img src="https://avatars.githubusercontent.com/u/29884391?s=80&v=4?s=100" width="100px;" alt="YaakovHunag920515"/><br /><sub><b>
YaakovHunag920515</b></sub></a><br /><a href="https://github.com/YaakovHunag920515/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Songoku1691"><img src="https://avatars.githubusercontent.com/u/102212067?s=80&u=46b32e68400dff7ee6083c243b3b6788b798563a&v=4?s=100" width="100px;" alt="Songoku1691"/><br /><sub><b>Songoku1691</b></sub></a><br /><a href="https://github.com/Songoku1691/songokutictactoe.git" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Timssse"><img src="https://avatars.githubusercontent.com/u/110025936?s=80&v=4?s=100" width="100px;" alt="Timssse"/><br /><sub><b>
Timssse</b></sub></a><br /><a href="https://github.com/Timssse/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/LLoyD1337"><img src="https://avatars.githubusercontent.com/u/99583480?s=80&v=4?s=100" width="100px;" alt="LLoyD1337"/><br /><sub><b>
LLoyD1337</b></sub></a><br /><a href="https://github.com/LLoyD1337/Aleo2" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/VeranOAS"><img src="https://avatars.githubusercontent.com/u/103969183?s=80&u=f737e0ca182789e0fc4fb57deebdf0439d4c30f7&v=4?s=100" width="100px;" alt="VeranOAS"/><br /><sub><b>VeranOAS</b></sub></a><br /><a href="https://github.com/VeranOAS/Raven-s-aleo" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/kirileshta"><img src="https://avatars.githubusercontent.com/u/129518667?s=80&v=4?s=100" width="100px;" alt="kirileshta"/><br /><sub><b>kirileshta</b></sub></a><br /><a href="https://github.com/kirileshta/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/dimapr1"><img src="https://avatars.githubusercontent.com/u/147644267?s=80&v=4?s=100" width="100px;" alt="dimapr1"/><br /><sub><b>
dimapr1</b></sub></a><br /><a href="https://github.com/dimapr1/tictactoe.git" title=“Tutorial>✅</a></td>
           <td align="center" valign="top" width="14.28%"><a href="https://github.com/senolcandir"><img src="https://avatars.githubusercontent.com/u/85374455?s=80&u=fad923f160c982ef28335592763b7fb9c0bc3aea&v=4?s=100" width="100px;" alt="senol10"/><br /><sub><b>
senol10</b></sub></a><br /><a href="https://github.com/senolcandir/senolcandir" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/hoangsoncomputer"><img src="https://avatars.githubusercontent.com/u/110523451?s=80&v=4?s=100" width="100px;" alt="hoangsoncomputer"/><br /><sub><b>hoangsoncomputer</b></sub></a><br /><a href="https://github.com/hoangsoncomputer/aleo_tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Timssse"><img src="https://avatars.githubusercontent.com/u/110025936?s=80&v=4?s=100" width="100px;" alt="Timssse"/><br /><sub><b>
Timssse</b></sub></a><br /><a href="https://github.com/Timssse/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
      <tr>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/Erskine2022"><img src="https://avatars.githubusercontent.com/u/145164260?s=80&u=92ddedf9be42988d8e067d3daa4b77c44d34b5d4&v=4?s=100" width="100px;" alt="Erskine2022"/><br /><sub><b>
Erskine2022</b></sub></a><br /><a href="https://github.com/Erskine2022/tictactoe" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/CTurE1"><img src="https://avatars.githubusercontent.com/u/93711669?s=80&v=4?s=100" width="100px;" alt="CTurE1"/><br /><sub><b>CTurE1</b></sub></a><br /><a href="https://github.com/CTurE1/aleo_lottery" title=“Tutorial>✅</a></td>
          <td align="center" valign="top" width="14.28%"><a href="https://github.com/HoratioElise"><img src="https://avatars.githubusercontent.com/u/145164393?s=80&u=50e5c69475f9769c167cbdaaa97a6a40c5708f8f&v=4?s=100" width="100px;" alt="HoratioElise"/><br /><sub><b>HoratioElise</b></sub></a><br /><a href="https://github.com/HoratioElise/tictactoe" title=“Tutorial>✅</a></td>
      </tr>
  </tbody>
  <tfoot>
    <tr>
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
