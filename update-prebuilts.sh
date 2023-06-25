#!/bin/bash

set -x -e

# Need to sync the default feature as in daemon/Cargo.toml
# OPT use --no-default-features allowing CI to disable default feature if needed.
BUILD_FEATURES=${BUILD_FEATURES:-"\
apps-service,\
audiovolumemanager-service,\
contacts-service,\
contentmanager-service,\
devicecapability-service,\
dweb-service,\
evm-service,\
geckobridge-service,\
powermanager-service,\
procmanager-service,\
settings-service,\
tcpsocket-service,\
time-service,\
virtual-host"}
BUILD_TYPE=${BUILD_TYPE:-"release"}
JS_BUILD_TYPE="prod"
OPT="--release --no-default-features"

if [[ "${BUILD_TYPE}" == "debug" ]]; then
    JS_BUILD_TYPE="build"
    BUILD_FEATURES="${BUILD_FEATURES},daemon"
    OPT=
elif [[ "${BUILD_TYPE}" == "beta" ]]; then
    BUILD_TYPE="release"
    BUILD_FEATURES="${BUILD_FEATURES},daemon"
fi

pushd daemon > /dev/null
FEATURES=${BUILD_FEATURES} ./xcompile.sh ${OPT}
popd > /dev/null

if [[ "${BUILD_APPSCMD}x" != "x" ]]; then
    pushd services/apps/appscmd > /dev/null
    export IS_BUILDING_APPSCMD=yes
    ./xcompile.sh ${OPT}
    popd > /dev/null
fi


TARGET_ARCH=${TARGET_ARCH:-armv7-linux-androideabi}

mkdir -p prebuilts/${TARGET_ARCH}
cp ./target/${TARGET_ARCH}/${BUILD_TYPE}/api-daemon prebuilts/${TARGET_ARCH}/api-daemon
if [[ -f ./target/${TARGET_ARCH}/${BUILD_TYPE}/appscmd ]]; then
    cp ./target/${TARGET_ARCH}/${BUILD_TYPE}/appscmd prebuilts/${TARGET_ARCH}/appscmd
fi
# We don't build symbols for all targets
if [[ -d ./target/${TARGET_ARCH}/${BUILD_TYPE}/symbols ]]; then
    cp -rf ./target/${TARGET_ARCH}/${BUILD_TYPE}/symbols prebuilts/${TARGET_ARCH}/
fi

# Update the client side libs and move them to the right place.
BUILD_TYPE=${JS_BUILD_TYPE} RELEASE_ROOT=./prebuilts/http_root/api/v1 ./release_libs.sh
