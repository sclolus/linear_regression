#!/bin/bash

function random_float {
	local min=$1
	local max=$2
	local seed=$(od -An -tu4 -N4 /dev/urandom | tr -d ' ')
	awk -v seed="$seed" -v min=$min -v max=$max 'BEGIN{srand(seed); printf "%.6f", min+rand()*(max-min)}'
}

while [ true ] ;
do
	THETA0=$(random_float 0 30000)
	THETA1=$(random_float -1 0)
	MILEAGE=$(random_float 0 500000)
	

	echo "$THETA0,$THETA1" > weights
	echo $MILEAGE | cargo run
done;

	
