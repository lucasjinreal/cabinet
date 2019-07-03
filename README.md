# Cabinet

![](https://i.loli.net/2018/01/14/5a5b4d6d28d5e.jpeg)
![](https://i.loli.net/2018/01/14/5a5b4ef2a4fb2.jpeg)

This my cabinet tool box. It implement in *Rust*, and do many helper things for me. Such as:

- Help me generate a C++ project;
- Help me new a blog template;
- Help me get some pictures which I want;
- Help me simplified the *git* process, so that not need input every commit message;
- More things...
**cabinet** is an ultimate tool for me to accelerate my daily work. It's fast written in rust!


## Updates

**2019.07.03**: I am trying to adding git submodule support for *cabinet*, so that I can simplify daily git push issues.


## Install

To install **cabinet**, you should install *rust* first. Then you can build it by:
    
```
cargo install
sudo cp ~/.cargo/bin /usr/local/bin
```
Or, you can directly using:

```
./install_carbinet.sh
```
To automatically install built binary carbinet according to your system.


## Usage

**cabinet** do many things in my daily work. You can using it like this:
    
```
// this command will upload your local git repo to remote, if you set
// your github username and password in ~/.cabinet/cabinet.toml
// same as: 'git add . && git commit -am 'comment' && git push origin master'
cabinet git 'just a comment'

// generate a blog template as you wish
cabinet blog 'I want generate a blog template'
// generate blog with a filename start with date
cabinet blog -d 'Another blog'

// generate a sample project with CMake in C++, more language will be supported
cabinet code -cc 'hello'

// more function are adding....
```


## Copyright

**cabinet** first developed by *Jin Fagang*, All Rights Reserved.



