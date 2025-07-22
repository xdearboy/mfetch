pkgname=mfetch
pkgver=0.1.0
pkgrel=1
pkgdesc="Minimal system info fetcher"
arch=('x86_64')
url="https://github.com/xdearboy/mfetch"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("https://github.com/xdearboy/mfetch/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('d5558cd419c8d46bdc958064cb97f963d1ea793866414c025906ec15033512ed')

build() {
  cd "$srcdir/mfetch-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/mfetch-$pkgver"
  install -Dm755 "target/release/mfetch" "$pkgdir/usr/bin/mfetch"
}
