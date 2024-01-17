################################################################
### Git Aliases
################################################################
alias ss='git status'
alias log="git log --all --decorate --oneline --graph"
alias pick='git cherry-pick'
alias gs='git config --global user.name \$NAME && git config --global user.email \$EMAIL'

alias aa='git add .'
alias cam='git commit -a -m'
alias wip="git add . && git commit -m 'wip'"

alias p='git pull'
alias pp="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias pt='git push --tags'

# checkout aliases
alias cm='git checkout main'
alias cb='git checkout -b'
alias cr='f() { git checkout -b $1 origin/$1; }; f'

# Remove aliases
alias rmb="git branch --merged | egrep -v '(^\*|main|master)' | xargs git branch -d"
alias rmt="git tag --merged | egrep -v '(^\*|main|master)' | xargs git tag -d"
alias rmd="git rm -r \$(git ls-files --deleted)"

# Undo all
alias nah="git reset --hard;git clean -df"

# Tag Version / Overwrite Tag Version
function v() {
    if [ -z "$1" ]; then
        echo "No tag/version provided"
        return 1
    fi
    git tag -a $1 -f -m 'Bump to $1'
    git push origin $1 --force
}

################################################################
### Other Aliases
################################################################
alias c=clear
alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'
alias nrs='npm run start'

alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'
alias db='psql $DATABASE_URL'

# List files and directories

################################################################
### Quick Shortcut Editting Config Files
################################################################

alias cfa='code $HOME/.bash_aliases'
alias cfb='code $HOME/.bashrc'
alias cfp='code $HOME/.bash_profile'

################################################################
### File System Aliases
################################################################
alias cf='cd $HOME/.config && ls -a'
alias h='cd $HOME -a'
alias w='cd /workspace -a'

alias ..='cd ..'
alias ...='cd ../..'
alias .3='cd ../../..'
alias .4='cd ../../../..'
alias .5='cd ../../../../..'

alias ls='ls -l'
alias ll='ls -lah'

################################################################
### File permissions
################################################################
alias chx="chmod +x"
alias chax="chmod a+x"
alias 000="sudo chmod 000"
alias 555="sudo chmod 555"
alias 600="sudo chmod 600"
alias 644="sudo chmod 644"
alias 750="sudo chmod 750"
alias 755="sudo chmod 755"
alias 775="sudo chmod 775"
alias 777="sudo chmod 777"


################################################################
### Source Config Files Quickly
################################################################


function .b() {
    echo "Sourcing bash config files"
    if [ -f "$HOME/.bash_profile" ]; then
        source $HOME/.profile
    fi
    if [ -f "$HOME/.bashrc" ]; then
        source $HOME/.bashrc
    fi
    if [ -f "$HOME/.bash_aliases" ]; then
        source $HOME/.bash_aliases
    fi
}

function generate() {
    if [ "$1" == "server" ]; then
        cargo generate codeitlikemiley/server_template --name "$2"
    elif [ "$1" == "service" ]; then
        cargo generate codeitlikemiley/services_template --name "$2"
    else
        echo "Invalid option. Please use 'server' or 'service'."
    fi
}

