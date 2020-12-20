prefix = /usr/local

all:
	cargo build --release

install:
	setcap cap_net_raw,cap_net_admin=eip target/release/ipmap
	install target/release/ipmap $(DESTDIR)$(prefix)/sbin
	install data/ipmap.1 $(DESTDIR)$(prefix)/share/man/man1

deb-gen:
	rm -rf build-deb/

	mkdir build-deb/

	tar -czvf ./build-deb/ipmap_0.1.6.orig.tar.gz data/ src/ Cargo.toml LICENSE README.md Makefile

	mkdir ./build-deb/ipmap_0.1.6/

	tar -xvf ./build-deb/ipmap_0.1.6.orig.tar.gz -C ./build-deb/ipmap_0.1.6/

	cp -rf ./debian/ ./build-deb/ipmap_0.1.6/

	cd ./build-deb/ipmap_0.1.6/ && 	debuild -us -uc

arch-gen: 
	rm -rf build-arch/
	mkdir build-arch/
	tar -czvf ./build-arch/ipmap-0.1.6.tar.gz data/ src/ Cargo.toml LICENSE README.md Makefile

deb-clean:
	rm -rf build-deb/

clean:
	cargo clean
