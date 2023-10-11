#!/bin/bash
set -e

TMP_FOLDER=./.stories

# Cleanup
rm -rf $TMP_FOLDER
mkdir $TMP_FOLDER

# Building binaries
cargo build --release
echo

STORIES="./target/release/stories"
TOTAL=`xsv count $1`

echo "1. Extracting vocabulary"
echo "------------------------"
$STORIES vocab $1 --total $TOTAL --ngrams 2 > $TMP_FOLDER/vocab.csv
echo

echo "2. Determining window size"
echo "--------------------------"
WINDOW=`$STORIES window $1 --raw --size 0.5 --total $TOTAL`
echo "Optimal window size should be: $WINDOW"
echo

echo "3. Applying clustering algorithm"
echo "--------------------------------"
$STORIES nn $TMP_FOLDER/vocab.csv $1 -w $WINDOW --total $TOTAL --ngrams 2  --threshold 0.65 > $TMP_FOLDER/nn.csv
echo

echo "4. Evaluating"
echo "-------------"
xsv join --left id $1 id $TMP_FOLDER/nn.csv | xsv select id,created_at,nearest_neighbor,thread_id,distance > $TMP_FOLDER/nn_dated.csv
$STORIES eval $2 $TMP_FOLDER/nn_dated.csv --total $TOTAL --datecol created_at
