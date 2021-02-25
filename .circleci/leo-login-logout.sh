# leo login & logout

$LEO new my-app && cd my-app || exit 1
$LEO login -u "$ALEO_PM_USERNAME" -p "$ALEO_PM_PASSWORD"
$LEO add howard/silly-sudoku
$LEO remove silly-sudoku
$LEO logout
