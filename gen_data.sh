#!/bin/bash
# Script to generate datasets for experiments
set -e

cardinalities=()
size_multiplies=(100)

for i in {1..60}; do
	cardinalities+=($((i * 10000)))
done

mkdir -p data results plots

for card in "${cardinalities[@]}"; do
	for mult in "${size_multiplies[@]}"; do
		size=$((card * mult))
		out="data/data_${card}_${size}.txt"
		echo "generating card=${card} size=${size} to ${out}"
		cargo run --release -q --manifest-path="gen_data/Cargo.toml" "${out}" ${card} ${size}
	done
done
