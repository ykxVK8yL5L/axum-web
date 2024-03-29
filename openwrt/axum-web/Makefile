include $(TOPDIR)/rules.mk

PKG_NAME:=axum-web
PKG_VERSION:=0.0.4
PKG_RELEASE:=$(AUTORELESE)

PKG_LICENSE:=MIT
PKG_MAINTAINER:=ykxVK8yL5L

PKG_LIBC:=musl
ifeq ($(ARCH),arm)
  PKG_LIBC:=musleabi

  ARM_CPU_FEATURES:=$(word 2,$(subst +,$(space),$(call qstrip,$(CONFIG_CPU_TYPE))))
  ifneq ($(filter $(ARM_CPU_FEATURES),vfp vfpv2),)
    PKG_LIBC:=musleabihf
  endif
endif

PKG_ARCH=$(ARCH)
ifeq ($(ARCH),i386)
  PKG_ARCH:=i686
endif

PKG_SOURCE:=axum-web-v$(PKG_VERSION).$(PKG_ARCH)-unknown-linux-$(PKG_LIBC).tar.gz
PKG_SOURCE_URL:=https://github.com/ykxVK8yL5L/axum-web/releases/download/v$(PKG_VERSION)/
PKG_HASH:=skip

include $(INCLUDE_DIR)/package.mk

define Package/$(PKG_NAME)
	SECTION:=multimedia
	CATEGORY:=Multimedia
	TITLE:=Axum-web
	URL:=https://github.com/ykxVK8yL5L/axum-web
endef

define Package/$(PKG_NAME)/description
A demo of website.
endef

define Package/$(PKG_NAME)/conffiles
/etc/config/axum-web
endef

define Download/sha256sum
	FILE:=$(PKG_SOURCE).sha256
	URL_FILE:=$(FILE)
	URL:=$(PKG_SOURCE_URL)
	HASH:=skip
endef
$(eval $(call Download,sha256sum))

define Build/Prepare
	mv $(DL_DIR)/$(PKG_SOURCE).sha256 .
	cp $(DL_DIR)/$(PKG_SOURCE) .
	shasum -a 256 -c $(PKG_SOURCE).sha256
	rm $(PKG_SOURCE).sha256 $(PKG_SOURCE)

	tar -C $(PKG_BUILD_DIR)/ -zxf $(DL_DIR)/$(PKG_SOURCE)
endef

define Build/Compile
	echo "$(PKG_NAME) using precompiled binary."
endef

define Package/$(PKG_NAME)/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/axum-web $(1)/usr/bin/axum-web
	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) ./files/axum-web.init $(1)/etc/init.d/axum-web
	$(INSTALL_DIR) $(1)/etc/config
	$(INSTALL_CONF) ./files/axum-web.config $(1)/etc/config/axum-web
endef

$(eval $(call BuildPackage,$(PKG_NAME)))
