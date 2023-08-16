pkgname=danicampa-turn-off-display
pkgver=1.0
pkgrel=1
epoch=
pkgdesc="Simple command to turn off your macbook air (2014) display"
arch=('i686' 'x86_64')
url="https://github.com/danicampa90/TODO"
license=('GPL')
groups=()
depends=()
makedepends=('rust' 'make')
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=(!libtool !staticlibs !emptydirs)
install=
changelog=
source=("git+file://${PWD}") # "$pkgname-$pkgver.tar.gz"
noextract=()
md5sums=()
validpgpkeys=()

#prepare(){}

build() {
#	cd "$pkgname-$pkgver"
	pwd
	make
}

#check() {}

package() {
#	cd "$pkgname-$pkgver"
	make DESTDIR="$pkgdir/" install
}
