DESCRIPTION = "Isar target filesystem"

PV = "0.1"

inherit image

# network-manager necessary for auto internet connection
# iputils-ping for ping command to test connection
IMAGE_PREINSTALL += " \
    network-manager \
    iputils-ping \
"
