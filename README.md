# t2meili
convert torrent file (bencode) metadata to json lines, to
organize your linux isos with a search engine like
[meilisearch](https://github.com/meilisearch/meilisearch)

`t2meili` [FILE]...

## json lines?
this format makes output [easier to process](https://jsonlines.org/).
for example, this allows chopping up output with `split(1)`
into managable chunks to avoid payload size limits, or
adding together multiple small runs with `cat(1)`

for stuff that does not support json lines, the `jq(1)`
utility may be used to turn it into a normal json array:
```
jq -sc
```

