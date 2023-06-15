# singerie
Clustering of textual documents with time window


## How to install

1. Install cargo [(see cargo documentation)](https://doc.rust-lang.org/cargo/getting-started/installation.html).

2. Install singerie

```bash
cargo install --git https://github.com/medialab/singerie.git
```

## How to run

### Extract vocabulary
```
singerie vocab my_file.csv --ngrams 2 > my_vocab.csv
```

### Determine time window
```
WINDOW=`singerie window my_file.csv --raw`
```

### Apply clustering algorithm
```
singerie nn my_vocab.csv my_file.csv -w $WINDOW --ngrams 2  --threshold 0.65 > nn.csv
```

### Evaluate cluster quality
```
xsv join --left id my_file.csv id nn.csv | xsv select id,created_at,nearest_neighbor,thread_id,distance > nn_dated.csv
singerie eval my_labels.csv nn_dated.csv --datecol created_at
```
