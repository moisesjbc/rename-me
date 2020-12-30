# rename-me (Yeah, working on the name)

This is a side project of mine for trying to implement DRY principle (Don't repeat yourself) for describing my games and other projects on multiple platforms. Also, it is my excuse for learning Rust ;-)

An use case for the first point is my game "[UFO taxi!](https://github.com/moisesjbc/ufo-taxi)". I (will) have a README, the description page on Ludum Dare, the itch.io page, etc all of them presenting the same info but with a different format. I am aiming to have all the data centralized in one single JSON file and then by using different templates simply automatically render the descriptions for all the enumerated places.

At the time of writing this I am seeing this as a side / toy project that might evolve into something cool or not. What I mean to say is don't expect fast advances as for now ;-)

## Usage

Sample:

        cargo run hello-world.html samples/hello-world.json
