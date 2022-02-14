# Extract subs

Small application written in Rust that extracts the subs of a set of MKV files, using `mkvextract` and `ffmpeg`.

## Installation

1. Clone this repository into yout local machine.

2. Browse to the directory of the project via Terminal, and install the package:

```
cargo install --path ./
```

3. Assure the global cargo `bin` folder is in your `$PATH` variable.

## Usage

### Method 1: Current dir
1. Browse via Terminal to the folder that contains your MKV files
2. Run: `extract-subs`

### Method 2: Specify dir
1. Run: `extract-subs /path/to/the/dir/to/extract`
