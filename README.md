# Birdhouse
Free and open source federated social media, designed to resemble Twitter. Built on ActivityPub & interoperable with Mastodon (+ others).

## Features

### Built on ActivityPub
Birdhouse is built with [ActivityPub](https://www.w3.org/TR/activitypub/) allowing full interoperability with any other service that uses the protocol, and is specifically built to meet the spec requirements for [Mastodon](https://docs.joinmastodon.org/spec/activitypub/).

### Familiar UI
Designed to resemble modern Twitter (pre-2022), Birdhouse aims to be simple to understand and use as well as familiar for any users jumping ship to the Fediverse.

### Designed for Scale
Fully constructed with the Rust programming language and designed to be horizontally scaled. Birdhouse can be easily deployed to a Kubernetes cluster or a similar container orchestration service.

## Motivation

I believe in the power of the Fediverse, interoperable social networks, and FOSS. While terms like "federated", "ActivityPub" and "decentralized" might sound awesome to nerds, the average person looking to leave the sinking ship of Twitter may view services such as Mastodon to be too complex to understand for a casual social media user (even when in reality, it's not as bad as it seems). Additionally, with the incredible failure of "Web3" and adjacent cryptocurrency projects, I've had difficulty communicating the power of decentralization to someone who's only experience with the concept is scam NFT projects.

Birdhouse aims to make the transition to decentralization easier for folks who aren't power users or are tech illiterate. The project is designed to be as simple as possible for regular users to operate, while also packing an incredibly powerful backend for admins and people who know their stuff.

### Current status/Future goals

Currently Birdhouse is still under development. You shouldn't run this in production. Only the web app is being worked on currently to bring it fully up-to-spec with ActivityPub and additional requirements from Mastodon.

A mobile client for iOS and Android is planned for the future.

## Deployment

### Tech Stack:
- **Rust** for all backend services.
- **Svelte (TS)** for the frontend web app.
- **MySQL** as a database. Note that anything compatible with MySQL, such as Vitess, will work fine.

## Contributing

This monorepo contains the REST API (`api/`) and Svelte app (`web/`).