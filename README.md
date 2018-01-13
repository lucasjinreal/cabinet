# Cabinet

This my cabinet tool box. It implement in *Rust*, and do many helper things for me. Such as:

- Help me generate a C++ project;
- Help me new a blog template;
- Help me get some pictures which I want;
- Help me simplified the *git* process, so that not need input every commit message;
- More things...

**cabinet** is an ultimate tool for me to accelerate my daily work. It's fast written in rust!


# Usage

**cabinet** do many things in my daily work. You can using it like this:
    
```
// this command will upload your local git repo to remote, if you set
// same as: 'git add . && git commit -am 'comment' && git push origin master'
// your github username and password in ~/.cabinet/cabinet.toml
cabinet git 'just a comment'

// generate a blog template as you wish
cabinet blog 'I want generate a blog template'
// generate blog with a filename start with date
cabinet blog -d 'Another blog'

// generate a sample project with CMake in C++, more language will be supported
cabinet code -cc 'hello'

// more function are adding....
```


# Copyright

**cabinet** first developed by *Jin Fagang*, All Rights Reserved.



