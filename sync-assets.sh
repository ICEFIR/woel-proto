#!/bin/sh
echo -e "Bi-Syncing assets from b2 storage"
rclone bisync assets woel-bevy:woel-bevy $@
