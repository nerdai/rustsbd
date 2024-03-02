# RustSBD

## Developing

While in pre-release, we have not published macros crate and so we need
to install it from github.

You won't be able to run `Cargo install` unless you have ssh-agent running
that adds your ssh key used for Github.

```sh
eval `ssh-agent -s`
ssh-add
```
