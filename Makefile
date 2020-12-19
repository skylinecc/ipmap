all:
	cargo +nightly build --release

deb:
	rm -rf build-deb/

	mkdir build-deb/

	tar -czvf ./build-deb/ipmap_0.1.6.orig.tar.gz data/ src/ Cargo.toml LICENSE README.md

	mkdir ./build-deb/ipmap_0.1.6/

	tar -xvf ./build-deb/ipmap_0.1.6.orig.tar.gz -C ./build-deb/ipmap_0.1.6/

	cp -rf ./debian/ ./build-deb/ipmap_0.1.6/

deb-clean:
	rm -rf build-deb/

clean:
	cargo clean
