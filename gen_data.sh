#!/bin/bash

cardinalities=()

# large datasets

size_multiplies=(100)
for i in {1..50}; do
	cardinalities+=($((i * 10000)))
done

# small datasets

#size_multiplies=(10)
#for i in {1..2000}; do
	#cardinalities+=($((i)))
#done

# generate the datasets

for card in "${cardinalities[@]}"; do
	for mult in "${size_multiplies[@]}"; do
		size=$((card * mult))
		out="data/data_${card}_${size}.txt"
		echo "generating card=${card} size=${size} to ${out}"
		cargo run --release -q --manifest-path="gen_data/Cargo.toml" "${out}" ${card} ${size}
	done
done
