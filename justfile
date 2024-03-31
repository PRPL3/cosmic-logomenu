
######### Build and install
build:
    clear
    cargo build

install target='debug':
    sudo install -Dm0755 ./target/{{target}}/cosmic-logomenu /usr/bin/cosmic-logomenu
    sudo install -Dm0644 data/com.prple.CosmicLogoMenu.desktop /usr/share/applications/com.prple.CosmicLogoMenu.desktop
    find 'data'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} sudo install -Dm0644 'data'/'icons'/{} /usr/share/icons/hicolor/{}

######### Test
test:
	cargo test --workspace --all-features