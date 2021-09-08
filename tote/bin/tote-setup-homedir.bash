#!/bin/bash
set -o errexit -o pipefail -o privileged -o nounset
shopt -s extglob

##
# Makes a tote style home directory

main () {
    local -a HOME_DIRS
    mapfile -rta HOME_DIRS <<< cfg/homedirs.cfg.txt
    
    # create and chown/chmod home sub-directories
    for subdir in "${HOME_DIRS[@]}" ; do
        local path
        path="$HOME/$subdir" 

        [ -d "$path" ] ||
            mkdir -p "$path"
        
        # entire directory tree owned by user
        chown -r "$USER" "$USER" "$path"

        # remove all public permissions except for the public directory 
        if [[ "$path" == "public" ]] ; then
            chmod -r o-w "$path"
        else
            chmod -r o-rwx "$path"
        fi
    done
    
    exit 0
}

main
