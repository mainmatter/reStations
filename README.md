# reStations

Rust-written REST-API wrapper around [trainline-eu/stations](https://github.com/trainline-eu/stations), a 'list of stations and associated metadata'. Hence the name.

This is an educational project we're working on during Mainmatter's internal Rust sessions

This system consists of two applications:
- `restations-cli`: A CLI that fetches the latest version of the `stations.csv` file in the stations repo and saves it as a sqlite database file.
- `restations-server`: A REST API the serves the data saved by `restations-cli` from memory.

The initial version of `restations-server` will have the data compiled in its binary, so that it can be served and scaled trivially. This comes at the cost of needing to deploy the latest version each time the latest version of `stations.csv` gets synced, but it saves us having to put the data in a database or a persistent storage.

Later versions may support loading the data periodically at runtime. 


## Open questions
- Why PollSender