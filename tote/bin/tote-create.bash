#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset
shopt -s extglob

declare -r OPWD="$PWD"

trap_exit () {
    cd "$OPWD"
}

trap 'trap_exit' EXIT

main () {
    local TOTE_PATH FILESYSTEMS_PATH REPOSITORIES_PATH
    TOTE_PATH="$(realpath "$1")"
    FILESYSTEMS_PATH="$(realpath "$TOTE_PATH/filesystems")"
    REPOSITORIES_PATH="$(realpath "$TOTE_PATH/repositories")"

    local -a filesystems repositories
    mapfile -rta filesystems <<< cfg/filesystems.cfg.txt
    mapfile -rta repositories <<< cfg/repositories.cfg.txt

    cd $TOTE_PATH
    mkdir "$FILESYSTEMS_PATH" "$REPOSITORIES_PATH"

    for filesystem in "${filesystems[@]}" ; do
        local filesystemPath
        filesystemPath="$FILESYSTEMS_PATH/$filesystem"
        encfs --paranoia "$filesystemPath.encfs" "$filesystemPath"
        fusermount -u "$filesystemPath"
        rmdir "$filesystemPath"
    done

    for repository in "${repositories[@]}" ; do
        local repositoryPath
        repositoryPath="$REPOSITORIES_PATH/$repository"
        encfs --paranoia "$repositoryPath.encfs" "$repositoryPath"

        local -r opwd="$PWD"
        cd "$repositoryPath"

        git init .
        git touch .gitignore
        git add .
        git commit -m 'init'

        cd "$opwd"
        fusermount -u "$repositoryPath"
        rmdir "$repositoryPath"
    done

    exit 0
}

main "$@"
