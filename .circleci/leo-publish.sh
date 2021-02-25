# leo login, publish and logout

$LEO new test-app && cd test-app
$LEO login -u "ALEO_PM_USERNAME" -p "ALEO_PM_PASSWORD"

# sed command below takes 0.1.0 version (which is default for new programs)
# and replaces it with GITHUB_RUN_ID - a unique incremental number for each
# GH Actions run; [AUTHOR] gets replaced with $USER variable; and we're ready
# to publish package with newer version and correct author
cat Leo.toml | sed "s/0.1.0/0.1.$GITHUB_RUN_ID/g" | sed "s/\[AUTHOR\]/$USER/g" > Leo.toml

$LEO publish
$LEO logout

mkdir hello-world && cd hello-world || exit 1
$LEO init
ls -la
$LEO run

