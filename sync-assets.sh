#!/bin/sh
echo -e "Bi-Syncing assets from b2 storage"
rclone bisync woel-bevy:woel-bevy assets $@
