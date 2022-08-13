#!/bin/bash

input=$(cat $1)
file=$1
path_without_ext=${file%.*.*}
ext=${file##*.}
worse=$(rustcii decode "$input")
echo $worse > "${path_without_ext}.worse.${ext}" 