prefix = /usr/local
version = 0.1.6

all:
	cargo build --release

install:
	ls target/
	sudo setcap cap_net_raw,cap_net_admin=eip target/release/ipmap
	install target/release/ipmap $(DESTDIR)$(prefix)/sbin
	install data/ipmap.1 $(DESTDIR)$(prefix)/share/man/man1


uninstall:
	rm -fv $(prefix)/sbin/ipmap
	rm -fv $(prefix)/share/man/man1/ipmap.1

deb-gen:
	rm -rf build-deb/

	mkdir build-deb/

	tar -czvf ./build-deb/ipmap_$(version).orig.tar.gz data/ src/ Cargo.toml LICENSE README.md Makefile

	mkdir ./build-deb/ipmap_$(version)/

	tar -xvf ./build-deb/ipmap_$(version).orig.tar.gz -C ./build-deb/ipmap_$(version)/

	cp -rf ./packaging/debian/ ./build-deb/ipmap_$(version)/

	cd ./build-deb/ipmap_$(version)/ && 	debuild -us -uc

arch-gen: 
	cargo build --release
	
	tar -czf ./packaging/arch/ipmap-$(version).tar.gz data/ LICENSE README.md Makefile ./target/release/ipmap
		
	cargo clean

build-clean:
	rm -rf build-*

clean:
	cargo clean
