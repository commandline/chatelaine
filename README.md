# chatelaine

A lightweight JWK server, written in Rust and designed to be dev ops friendly,
i.e. containerized and easily configured.

## What is JWK?
From the [RFC](https://tools.ietf.org/html/rfc7517)
> A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure
> that represents a cryptographic key.

Using JSON to represent a key makes it fairly portable. Most languages and
stacks have good support for JSON and for cryptographic keys. Some even have
good libraries for JWK. JSON is more amenable to sharing over a network,
between different services and applications than other formats for encoding
cryptographic keys that may introduce formatting differences between platforms
or may not have consistent support. Even if your stack of choice doesn't have
direct support for JWK, it should be approachable to work directly with the
JSON and your cryptographic library of choice.

## Why a JWK server?

If you are working with JSON Web Tokens, especially signed (JWS) or encrypted
(JWE), then having a way to securely distribute keys is important. You will
likely encounter JWT when working with authentication and authorization
protocols and services, like [OpenId Connect](https://openid.net/connect/),
[OAuth 2.0](https://oauth.net/2/), and [Auth0](https://auth0.com/#!).

If you adhere to the [12 Factor app methodology](https://12factor.net/), then
you may try placing keys in environment variables. For private key material
this may be less than ideal, exposing a key via management tools and support
tools, unless you use a platform that has the means to strongly protect runtime
configurations of deployed services, including encryption at rest and access
controls. Even on a platform that allows you to control access and secure key
material where needed, deployment formats like Docker don't allow for changing
environment variables after start up. If you need to revoke, replace or add a
key, this places an undue burden on services needing cryptographic keys.

Using a dedicated service for key distribution allows key revocation,
replacement and addition to be much simpler. You can use whatever approach you
prefer for service location so that your service or application can connect to
the JWK server. Ideally a JWK server would provide stable URIs based on key ID
and perhaps some concept of a user or client, responding with clear, consistent
responses, e.g. 404 if there is no such key, 401 to trigger authentication, 403
if the user or client has insufficient privileges to retrieve a particular key.

## Lightweight?

You could use a configuration service, like etcd, or a database of some kind.
You would like end up add a fair amount on top of such a system specific to
managing and distributing keys. You could use something like the OpenID Connect
server, which is actually a whole suite of service, some of which you may not
need, especially if you already have good support in your applications for your
authentication or SSO approach of choice. Sometimes all you want is a simple,
secure way to manage and share the cryptographic keys you need, that's all.

## TODO
* [x] Pick a web framework - Iron, Rocket, etc.?
* [ ] Implement access control
  * [x] Model a user
  * [x] Add authentication to requests
  * [ ] Implement digest/salt on passwords
  * [ ] Look into digest auth in place of basic
  * [ ] Add an admin bit to user model
  * [ ] Add list user endpoint, limit to admin user
  * [ ] Write PUT endpoint for user
  * [ ] Write DELETE endpoint for user
* [ ] Add logging
* [ ] Implement JWK upload and storage
  * [ ] Add dependency on Medallion branch with JWK support
  * [ ] Add put endpoint
  * [ ] Figure out key ID handling - caller sets? create in order?
* [ ] Implement JWK serving
* [ ] Support key generation?
