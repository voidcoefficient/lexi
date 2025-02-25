# the `lexi` protocol

lexi is a secure protocol inspired by HTTPS that aims to enable hypertezt markdown. historically, markdown projects (wikis, blogs, etc) transformed its markdown files into HTML that were then served and consumed by web browsers. lexi removes the tranformation layer and serves markdown directly, empowering clients to render the markdown how they want it, not how the CSS dictates. this lets you control the experience you have when reading and navigating hypertext. a similar experience may happen when using RSS readers, although that's not quite the same thing.

## roadmap

current focus is for lexi to reach its MVP state.

### MVP

this aims to give an overview of what lexi is.

- [ ] first edition of lexi's protocol
  - [ ] server library in rust
  - [ ] server binary in rust
  - [ ] client library in rust
  - [ ] client library in go
  - [ ] client binary in go
- [ ] lexi RFC?
