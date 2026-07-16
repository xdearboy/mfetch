pkgname=mfetch
pkgver=0.5.0
pkgrel=1
pkgdesc="📦 memory-focused system info tool written in Rust."
arch=('x86_64')
url="https://github.com/org-fe/mfetch"
license=('MIT')
depends=('i2c-tools')
makedepends=('rust' 'cargo')
source=("https://github.com/org-fe/mfetch/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$srcdir/mfetch-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/mfetch-$pkgver"
  install -Dm755 "target/release/mfetch" "$pkgdir/usr/bin/mfetch"
}
