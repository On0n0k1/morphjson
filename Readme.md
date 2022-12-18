# Morphjson

A small script that reads names from a few json files, then save all of them together in 'names.json'.

Input files only have string names. Output file has entries of the following type.

```
{
    "name": String,
    "times": u32,
}
```

`times` is a randomly generated integer.

