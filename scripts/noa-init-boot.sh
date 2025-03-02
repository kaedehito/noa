#!/bin/sh

# ルートファイルシステムをマウント
#mount -t proc proc /proc
#mount -t sysfs sys /sys
#mount -t devtmpfs dev /dev

# 必要なカーネル機能を有効化
#echo 1 > /proc/sys/kernel/printk

# サービスを起動
printf "\x1b[36;1m==>\x1b[0m noa boot start\n"
./target/debug/noa boot start

# シェルを起動（デバッグ用）
exec /bin/sh




