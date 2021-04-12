# leo login & logout

$LEO login -u "$ALEO_PM_USERNAME" -p "$ALEO_PM_PASSWORD"
$LEO new my-app && cd my-app || exit 1

cat Leo.toml

# verify that in Leo.toml there's no line with [AUTHOR];
# since CI does not allow showing credentials, we won't see it in the file;
# so the only way to test is to make sure that there's just no [AUTHOR] there
[[ $(cat Leo.toml | grep "\[AUTHOR\]" | wc -l) -eq 0 ]] || exit 1

$LEO add howard/silly-sudoku
$LEO remove silly-sudoku
$LEO logout
