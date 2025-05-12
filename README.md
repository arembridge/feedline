# Feedline

## Description

Provide a list of files and this tool will ensure there is an empty line at the end of each file.  Simples.

### Example

```diff
def main():
    pass

if __name__=="__main__":
    main()
- no newline
+
```

## Usage

Provide a single file to reformat:

```bash
feedline dodgy_file.txt
```

Provide multiple files to reformat:

```bash
feedline  dodgy_file_1.txt dodgy_file_2.txt
```

Pipe files to reformat:

```bash
find ./src/ -type f | feedline
```

```bash
git diff --name-only main | feedline
```

Filter the results you care about:

```bash
ls | feedline --color=NEVER | grep '^SKIP'
```

## Installation

### Manual installation

1. Clone this repo: `git clone https://github.com/arembridge/feedline.git`
2. Install with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html): `cargo install --path .`

