#!/bin/bash

YEAR=$1
BASE_DIR=$2

MOD_TEMPLATE="mod.template.rs"
DAY_TEMPLATE="day.template.rs"

FOLDER_PATH="$BASE_DIR/y$YEAR"
MOD_FILE="$FOLDER_PATH/mod.rs"

MOD_DAYS="$(for val in {1..25}; do printf 'mod day%02d;' $val; done)"
SOLUTIONS="$(for val in {1..25}; do printf 'day%02d::Day%02d::new(),' $val $val; done)"
SOLUTIONS=${SOLUTIONS::-1}

mkdir -p $FOLDER_PATH
cp $MOD_TEMPLATE $MOD_FILE
sed -i "s/{{year}}/$YEAR/g" $MOD_FILE
sed -i "s/{{mod_days}}/$MOD_DAYS/g" $MOD_FILE
sed -i "s/{{solutions}}/$SOLUTIONS/g" $MOD_FILE

for day in {1..25}; do
    DAY_FILE="$FOLDER_PATH/$(printf 'day%02d.rs' $day)"
    cp $DAY_TEMPLATE $DAY_FILE
    sed -i "s/{{day}}/$(printf '%02d' $day)/g" $DAY_FILE
done

echo "$SOLUTIONS"
