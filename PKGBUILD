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
backup=("~/.config/organize-rt/organize-rt.toml")
options=()
install=
changelog=
source=(
        "$pkgname-$pkgver.tar.gz::https://gitlab.com/FixFromDarkness/organize-rt/-/archive/$pkgver/organize-rt-$pkgver.tar.gz")
noextract=()
sha256sums=("9a943b2d274bc78e84233a721a97d631962161a453810820f412e29f0b411cdf")
validpgpkeys=()

build() {
	cd $pkgname-$pkgver
	cargo build --release --locked
}

package() {
	cd $pkgname-$pkgver
	install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin/"
}
