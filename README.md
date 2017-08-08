badvestments
-----
Bad Investment Advice


### Motivation
I'm trying to learn more rust. To that end, I needed a relatively simple, self-contained project I
could conceivably implement in rust. Meanwhile, in a lighthearted conversation at lunch, I made a
joke that mortgage-backed bitcoin derivatives could be an investment someone would actually be
interested in. Thus the bad investment advice bot was born.

### badvestments.rules
`badvestments.rules` is a simple grammar describing how one might go from the abstract concept of a
`Badvestment` to a concrete phrase like "want to retire early? get deregulated timeshares". The
badvestment bot works by reading and parsing the grammar, starting with the root nonterminal
`Badvestment` and then repeatedly applying randomly-chosen applicable rules to nonterminals until
everything is a terminal. The sequence of terminals is the bad investment advice.

### badvestments.lalrpop
Somewhat "yo dawg"-ishly, `badvestments.lalrpop` itself describes a grammar that defines the
structure of the `badvestments.rules` file. The badvestment bot uses
[lalrpop](https://crates.io/crates/lalrpop) by [nikomatsakis](https://github.com/nikomatsakis) in a
build script to generate code that will parse `badvestments.rules`. That way I didn't have to write
the code to parse it myself.

### twitter-api
To go from locally amusing myself to amusing myself as the only follower of
[@badvestments](https://twitter.com/badvestments), a Twitter bot, I found the
[twitter-api](https://crates.io/crates/twitter-api) crate by [gifnksm](https://github.com/gifnksm).
Using the Twitter OAuth API, the library can read a timeline and post updates to it. In theory it
can generate new access tokens automatically (combined with manually visiting the URL it gives you
when signed in to the account you want to authorize it for), but when I tried this it created
read-only tokens, not read-write tokens, so that didn't work for me. I suspect I misunderstood the
API or Twitter or both. In the end I had to manually use the "Your Access Token" aspect of the
Twitter Application Management site ([apps.twitter.com](https://apps.twitter.com)).

Regardless, once the tokens had been properly generated, this library makes it a fairly simple
matter to generate a badvestment, marshall the correct tokens, and fire off a new tweet.
