# stories
Clustering of textual documents with time window. Rust implementation of the First Story Detection algorithm described in

    Allan, J., Lavrenko, V., Malin, D., & Swan, R. (2000, February). Detections, bounds, and timelines: Umass and tdt-3. In Proceedings of topic detection and tracking workshop (pp. 167-174). sn.

The default parameters have been tested for optimal results on tweets. This algorithm aims at **grouping together tweets adressing the same subject at the same moment**. In practice, we use a sliding time window to compute cosine distances between documents emitted within a given interval of time. For further details, please refer to:

    Mazoyer, B., Cagé, J., Hervé, N., & Hudelot, C. (2020, May). A french corpus for event detection on twitter. In Proceedings of the twelfth language resources and evaluation conference (pp. 6220-6227).




## How to install

1. Install cargo [(see cargo documentation)](https://doc.rust-lang.org/cargo/getting-started/installation.html).

2. Install stories

```bash
cargo install --git https://github.com/medialab/stories.git --locked
```

## How to run

Your data should be formatted in CSV format, with a `text` column and a `created_at` column. Other columns will be ignored.

### Extract vocabulary

This is an example of how to extract a vocabulary (i.e. the frequency of each term) from the textual documents.
```
stories vocab my_file.csv --ngrams 2 > my_vocab.csv
```
If possible, we recommend to compute vocabulary over a large period of time, even if the period of interest for the clustering is smaller.

To show all available parameters, use:

```
stories vocab --help
```

### Determine time window

This command computes the mean number of posts per day (from midnight to midnight) in the dataset, and uses this metric to compute the size of the sliding window in number of posts.

```
WINDOW=`stories window my_file.csv --raw`
```

The supported formats are the following:

    'Thu Jul 30 15:15:53 +0000 2026'
    'Thu Jul 30 15:15:53'
    '2026-07-30 15:15:53'
    '2026-07-30'
    1785417353 (UTC timestamp)



 To show all available parameters, use:

 ```
 stories window --help
 ```

### Apply clustering algorithm
```
stories nn my_vocab.csv my_file.csv -w $WINDOW --ngrams 2  --threshold 0.65 > nn.csv
```

 To show all available parameters, use:

 ```
 stories nn --help
 ```

### Evaluate cluster quality
```
xsv join --left id my_file.csv id nn.csv | xsv select id,created_at,nearest_neighbor,thread_id,distance > nn_dated.csv
stories eval my_labels.csv nn_dated.csv --datecol created_at
```
