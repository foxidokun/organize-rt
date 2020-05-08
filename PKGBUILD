# Maintainer: FixFromDarkness <fixfromdarkness@cock.li>
pkgname=organize-rt
pkgver=1.0.1
pkgrel=4
epoch=
pkgdesc="Organize file based on regex"
arch=('any')
url="https://gitlab.com/FixFromDarkness/organize-rt"
license=('GPLv3')
groups=()
depends=()
makedepends=(cargo)
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=()
noextract=()
sha256sums=()
validpgpkeys=()

build() {
	cargo build --release --locked
}

package() {
	install -Dm 755 ../target/release/${pkgname} -t "${pkgdir}/usr/bin/"
}
