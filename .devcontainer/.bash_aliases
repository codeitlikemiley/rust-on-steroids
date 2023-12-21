# Git aliases.
alias ss='git status'
alias gcm='git checkout main'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias pp="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias gcb='git checkout -b'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL'

# Cargo watch
alias cw='mold -run cargo watch --no-gitignore -i "*.scss" -i "*.ts" -i node_modules -x run'

# npm
alias nrs='npm run start'

# Database
alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'
alias db='psql $DATABASE_URL'

alias ls='ls -l'
alias ll='ls -lah'

function gta() {
    git tag -a $1 -f -m 'Bump to $1'
    git push origin $1 --force
}

function fport(){
lsof -i:$@
}



alias cfa='code $HOME/.bash_aliases'
alias cfb='code $HOME/.bashrc'
alias cfp='code $HOME/.bash_profile'
alias cf='cd $HOME/.config && ls -a'
alias h='cd $HOME -a'
alias w='cd /workspace -a'

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

alias ..='cd ..'
alias ...='cd ../..'
alias .3='cd ../../..'
alias .4='cd ../../../..'
alias .5='cd ../../../../..'
