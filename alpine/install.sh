#!/bin/bash
apk update
apk upgrade --available && sync

# Drivers
apk add linux-firmware wpa_supplicant bluez

GPU_OPTION='Please select your GPU choice: '
GPU_OPTIONS=('AMD Legacy (ATI)', 'AMD Radeon', 'Intel', 'Nvidia')
select opt in "${GPU_OPTIONS[@]}"
do
  case $opt in
    'AMD Legacy (ATI)')
      apk add xf86-video-ati
      ;;
    'AMD Radeon')
      apk add xf86-video-amdgpu linux-firmware-amdgpu
      ;;
    'Intel')
      apk add xf86-video-intel
      ;;
    'Nvidia')
      apk add xf86-video-nouveau
      ;;
    *) echo "invalid option $REPLY";;
  esac
done

# Desktop
apk add plasma-desktop systemsettings sddm breeze elogind polkit-elogind dbus
rc-update add dbus
rc-update add elogind
rc-update add polkit
rc-update add udev
rc-update add sddm

rc-update add wpa_supplicant boot

rc-service wpa_supplicant start
rc-service sddm start

# Applications
apk add firefox-esr

# XR Applications
apk add openvino opencv