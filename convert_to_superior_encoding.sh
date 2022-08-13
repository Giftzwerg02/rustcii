#!/bin/bash

input=$(cat $1)
file=$1
path_without_ext=${file%.*}
ext=${file##*.}
better=$(rustcii encode "$input")
echo $better > "${path_without_ext}.superior.${ext}" 