# Dataset approximation using linear least squares method
This app approximates set of points, defined in .csv format, with a line using linear least squares method.
## Usage
Run project with default configuration
```sh
cargo build
cargo run
```
If you wish to pass dataset location from command line, use:
```sh
cargo run -- --dataset ./myFile.csv
```
