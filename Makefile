.PHONY: do
do:
	cargo run
	sips -s format jpeg -s formatOptions 100 image.ppm --out image.jpg
	open image.jpg
