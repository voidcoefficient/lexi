# the `lexi` protocol

lexi is a **secure protocol** inspired by HTTPS that aims to enable **hypertext markdown**. historically, markdown projects (wikis, blogs, etc) transformed its markdown files into HTML/CSS files that were then served and consumed by web browsers. lexi removes this tranformation step and serves markdown directly. this empowers clients to render the markdown how they want it, not how the CSS dictates. **you** control the experience you have when reading markdown and navigating hypertext. a similar experience may happen when using RSS readers, although that's not quite the same thing.

## roadmap

current focus is for lexi to reach its MVP state.

### MVP

- [ ] first edition of lexi's protocol
  - [ ] server library in rust
  - [ ] server binary in rust
  - [ ] client library in rust
  - [ ] client library in go
  - [ ] client binary in go
- [ ] lexi RFC?
