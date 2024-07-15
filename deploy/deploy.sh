#!/bin/sh

# build locally and deploy to a target. assumes remote and local are the same arch and whatnot.
#
# setup:
# modify consts.sh accordingly
# create a systemd user service for ${TARGET}, `sheepy-moe`, to run ${TARGET_DIR}/server
# enable lingering for the user with such service

source deploy/consts.sh || exit $(echo "ensure cwd is root">&2)

SYSTEMD_DAEMON_HELPER='set XDG_RUNTIME_DIR=/run/user/$(id -u $(whoami))' # ssh is non-interactive

# build
rm -rf dist
trunk build --release || fail "build client"
cargo build --release --bin server --features ssr || fail "build server"
${SSH} ${TARGET} $(cat <<EOF
    ${SYSTEMD_DAEMON_HELPER}
    systemctl --user stop sheepy-moe
    sleep 0.5
EOF
) || fail "stop service"
${SCP} ${UPLOAD} ${TARGET}:${TARGET_DIR} || fail "copy to target"
${SSH} ${TARGET} $(cat<<EOF
    ${SYSTEMD_DAEMON_HELPER}
    cd ${TARGET_DIR}
    chmod +x server
    systemctl --user enable sheepy-moe
    systemctl --user restart sheepy-moe
    # fixme: handle if it didn't work
EOF
)
