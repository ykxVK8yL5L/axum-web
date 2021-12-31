include $(TOPDIR)/rules.mk

PKG_NAME:=luci-app-axum-web
PKG_VERSION:=1.0.0
PKG_RELEASE:=1
PKG_PO_VERSION:=$(PKG_VERSION)-$(PKG_RELEASE)

PKG_LICENSE:=MIT
PKG_MAINTAINER:=messense <messense@icloud.com>

LUCI_TITLE:=LuCI Support for axum-web
LUCI_PKGARCH:=all
LUCI_DEPENDS:=+axum-web +lua +libuci-lua

include $(TOPDIR)/feeds/luci/luci.mk

# call BuildPackage - OpenWrt buildroot signature
