
######### Build and install
build:
    clear
    cargo build

install target='debug':
    sudo install -Dm0755 ./target/{{target}}/cosmic-logomenu /usr/bin/cosmic-logomenu
    sudo install -Dm0644 data/com.prple.CosmicLogoMenu.desktop /usr/share/applications/com.prple.CosmicLogoMenu.desktop


######### Test
test:
	cargo test --workspace --all-features