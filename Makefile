prefix = /usr/local
version = 0.1.7

all:
	$(HOME)/.cargo/bin/cargo build --release

install:
	sudo setcap cap_net_raw,cap_net_admin=eip target/release/ipmap
	sudo install target/release/ipmap $(DESTDIR)$(prefix)/sbin/
	sudo install data/ipmap.1 $(DESTDIR)$(prefix)/share/man/man1/
	sudo install data/ipmap.desktop $(DESTDIR)/usr/share/applications/

uninstall:
	rm -fv $(DESTDIR)$(prefix)/sbin/ipmap
	rm -fv $(DESTDIR)$(prefix)/share/man/man1/ipmap.1
	rm -fv $(DESTDIR)$(prefix)/usr/share/applications/ipmap.desktop

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
	$(HOME)/.cargo/bin/cargo clean
