# EbcDict
*For when your hex dumps are way too large.*

## What's the point?
If you have seen a PEM dump you'll know that there is a large amount of ebcdic hex output, and it is difficult to read more than a small part at a time. Usually a PEM dump has readable text in it so it is worth translating it to debug. This is where Ebcdictionary comes in! just paste the hex in and plain text will be output. This repository contains a rust cli program as well as a python script if for some reason rust isn't your thing.

## Build Instructions
Simply enter:
``` bash
cargo build
```

## Execution Instructions
Invoke ebcdict with the binary either as a string or raw, spacing and case does not matter:
``` bash
ebcdict c8 85 93 93 96 6b 40 e6 96 99 93 84 5a
ebcdict "c8 85 93 93 96 6b 40 e6 96 99 93 84 5a"
ebcdict c8859393 966b40e6 96999384 5a
ebcdict c8859393966b40e6969993845a
```
