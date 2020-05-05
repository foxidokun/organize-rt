# Maintainer: FixFromDarkness <fixfromdarkness@cock.li>
pkgname=organize-rt
pkgver=0.9
pkgrel=1
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
backup=("~/.config/organize-rt/rules.toml")
options=()
install=
changelog=
source=(
        "$pkgname-$pkgver.tar.gz::https://gitlab.com/FixFromDarkness/organize-rt/-/archive/$pkgver/organize-rt-$pkgver.tar.gz")
noextract=()
md5sums=('skip')
validpgpkeys=()

build() {
	cd $pkgname-v$pkgver
	cargo build --release --locked
}

package() {
	cd $pkgname-v$pkgver
	install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin/"
}
