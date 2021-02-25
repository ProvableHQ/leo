# leo login, publish and logout

# Login
$LEO login -u "$ALEO_PM_USERNAME" -p "$ALEO_PM_PASSWORD"

# Clone the test-app package.
export PACKAGE="$ALEO_PM_USERNAME/test-app"
$LEO clone $PACKAGE
cd test-app || exit 1

# Fetch the current Leo package version number.
#
# 1. Print out the Leo.toml file.
# 2. Search for a line with the word "version".
# 3. Isolate that into a single line.
# 4. Split the line from the '=' sign and keep the right-hand side.
# 5. Remove the quotes around the version number.
# 6. Trim any excess whitespace.
export CURRENT=$(cat Leo.toml \
| grep version \
| head -1 \
| awk -F= '{ print $2 }' \
| sed 's/[",]//g' \
| xargs)

# Increment the current Leo package version number by 1.
#
# 1. Print out the Leo.toml file.
# 2. Search for a line with the word "version".
# 3. Isolate that into a single line.
# 4. Split the line from the '=' sign and keep the right-hand side.
# 5. Remove the quotes around the version number.
# 6. Trim any excess whitespace.
# 7. Increment the version number by 1 (on the semver patch).
#
# https://stackoverflow.com/questions/8653126/how-to-increment-version-number-in-a-shell-script
export UPDATED=$(cat Leo.toml \
| grep version \
| head -1 \
| awk -F= '{ print $2 }' \
| sed 's/[",]//g' \
| xargs \
| awk -F. -v OFS=. 'NF==1{print ++$NF}; NF>1{if(length($NF+1)>length($NF))$(NF-1)++; $NF=sprintf("%0*d", length($NF), ($NF+1)%(10^length($NF))); print}')

# Write the updated Leo package version number to the Leo.toml file.
export TOML=$(cat Leo.toml | sed "s/$CURRENT/$UPDATED/g")
echo $TOML > Leo.toml

# Run the package to confirm the manifest remains well-formed.
$LEO run

cat Leo.toml

# Publish the package to Aleo.pm
$LEO publish

# Logout
$LEO logout
