# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: FixFromDarkness <fixfromdarkness@cock.li>
pkgname=organize-rt
pkgver=0.8
pkgrel=1
epoch=
pkgdesc="Organize file based on regex"
arch=('any')
url="https://gitlab.com/"
license=('GPL')
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
source=(
        "$pkgname-$pkgver.tar.gz::https://gitlab.com/FixFromDarkness/organize-rt/-/archive/v0.8/organize-rt-v0.8.tar.gz")
noextract=()
md5sums=(3c1be3929332b535ac054e662fa0202a)
validpgpkeys=()

build() {
	cd $pkgname-v$pkgver
	cargo build --release --locked
}

package() {
	cd $pkgname-v$pkgver
	install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin/"
}
