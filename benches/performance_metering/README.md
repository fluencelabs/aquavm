# Preparing data for the tests

One may need to regenerate data on changing data format.  Here is the instruction for data regeneration.

## `parser_10000_100`

Data is empty, no regeneration required.

## `dashboard` and `network_explore`

Run `junk/gen_test_data`.  No WASM binary required.  It will generate prev and current data as JSON files
with prefixes `dashboard` and `explore`.

## `big_data` and `long_data`

TODO
