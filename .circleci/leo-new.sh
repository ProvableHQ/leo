$LEO new hello-world
ls -la
cd hello-world && ls -la

# verify that in Leo.toml there's a placeholder for author
# because at the time of calling `leo new` user is not logged in
[[ $(cat Leo.toml | grep "\[AUTHOR\]" | wc -l) -eq 1 ]] || exit 1

$LEO run
