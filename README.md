# singerie
Clustering of textual documents with time window


## How to install

You need to install cargo.

```
git clone https://github.com/medialab/singerie.git
cargo build --release
SINGERIE="./target/release/singerie"
```

## How to run

### Extract vocabulary
```
$SINGERIE vocab my_file.csv --ngrams 2 > my_vocab.csv
```

### Determine time window
```
WINDOW=`$SINGERIE window my_file.csv --raw`
```

### Apply clustering algorithm
```
$SINGERIE nn my_vocab.csv my_file.csv -w $WINDOW --ngrams 2  --threshold 0.65 > nn.csv
```

### Evaluate cluster quality
```
xsv join --left id my_file.csv id nn.csv | xsv select id,created_at,nearest_neighbor,thread_id,distance > nn_dated.csv
$SINGERIE eval my_labels.csv nn_dated.csv --datecol created_at
```
