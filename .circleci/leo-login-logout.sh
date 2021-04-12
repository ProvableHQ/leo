# leo login & logout

$LEO login -u "$ALEO_PM_USERNAME" -p "$ALEO_PM_PASSWORD"
$LEO new my-app && cd my-app || exit 1

cat Leo.toml
which wc

# verify that in Leo.toml there's a line with $ALEO_PM_USERNAME;
# because at the time of calling `leo new` user is logged in and we're expecting substitution
[[ $(cat Leo.toml | grep "\[$ALEO_PM_USERNAME\]" | wc -l) -eq 1 ]] || exit 1

$LEO add howard/silly-sudoku
$LEO remove silly-sudoku
$LEO logout
