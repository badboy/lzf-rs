#!/bin/bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cd target/doc

git init
git config user.name "Jan-Erik Rediger"
git config user.email "janerik@fnordig.de"

git remote add upstream "https://$GH_TOKEN@github.com/badboy/lzf-rs.git"
git fetch upstream && git reset upstream/gh-pages

touch .

cat <<EOF > index.html
<!doctype html>
<title>lzf</title>
<meta http-equiv="refresh" content="0; ./lzf/">
EOF

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
