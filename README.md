# Chapter Listing plugin for mdbook

The `chapter-list` preprocessor supports adding sub-chapter lists to documents.

## Example

With a `SUMMARY.md` file like:

```
- [Zoo Animals](./zoo.md)
  - [Large Cats](./cats.md)
    - [Lion](./lion.md)
    - [Tiger](./tiger.md)
  - [Zebra](./zebra.md)
  - [Turtle](./turtle.md)
```

Then include `<!-- chapter-list -->` in the zoo.md markdown file:

```md
## Animals in the Zoo:

<!-- chapter-list -->
```

The zoo.md file would be updated to:

```md
## Animals in the Zoo:

1. [Large Cats](./cats.md)
   1. [Lion](./lion.md)
   2. [Tiger](./tiger.md)
2. [Zebra](./zebra.md)
3. [Turtle](./turtle.md)
```


## Installation

Firstly add the following to your book's manifest file (usually `book.toml`)

```toml
[preprocessor.chapter-list]
```
