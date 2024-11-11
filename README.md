# Stations
Educational project during internal Rust sessions


### Project idea: API wrapper around https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv

- Clones the repo if it hasn’t been yet, or pulls the latest version on master daily if it does
- Loads the data from stations.csv into memory
- Exposes the data through a REST API (there might be a OSDM spec for it)
- 

### Potential implementations

- A lambda in AWS:
    - All data compiled into app during build process, loaded as part of the binary.
    - New binary generated every day. Generation process pulls the latest repo version.
    - Spares us having to put data in a DB.
    - Later on, support pulling the data repo daily and updating it on the fly
