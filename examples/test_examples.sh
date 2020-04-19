##!/bin/sh

for example in */; do
	echo Testing $example...
	cd $example; cargo check; cd ..;
done

