##!/bin/sh

cd ../../

for example in examples/*; do
	echo Testing $example...
	cd $example; cargo check; cd ../..;
done

