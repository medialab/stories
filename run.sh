#!/bin/bash
set -e

TMP_FOLDER=./.twembeddings

# Cleanup
rm -rf $TMP_FOLDER
mkdir $TMP_FOLDER

# Building binaries
cargo build --release
echo

TWEMBEDDINGS="./target/release/twembeddings"
TOTAL=`xsv count $1`

echo "1. Extracting vocabulary"
echo "------------------------"
$TWEMBEDDINGS vocab $1 --total $TOTAL > $TMP_FOLDER/vocab.csv
echo

echo "2. Determining window size"
echo "--------------------------"
WINDOW=`$TWEMBEDDINGS window $1 --raw --total $TOTAL`
echo "Optimal window size should be: $WINDOW"
echo

echo "3. Applying clustering algorithm"
echo "--------------------------------"
$TWEMBEDDINGS nn $TMP_FOLDER/vocab.csv $1 -w $WINDOW --total $TOTAL --threshold 0.7 > $TMP_FOLDER/nn.csv
echo

echo "4. Evaluating"
echo "-------------"
xsv join --left id $1 id $TMP_FOLDER/nn.csv | xsv select id,created_at,nearest_neighbor,thread_id,distance > $TMP_FOLDER/nn_dated.csv
$TWEMBEDDINGS eval $2 $TMP_FOLDER/nn_dated.csv --total $TOTAL --datecol created_at
