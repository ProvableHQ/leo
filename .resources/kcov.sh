cd /home/circleci/project/ &&
echo "---START_LIST---"
ls target/debug/deps
echo "---END_LIST---"
#for file in target/debug/deps/leo*-*[^\.d];
#  do
#    mkdir -p "target/cov/$(basename $file)";
#    echo "Processing target/cov/$(basename $file)"
#    /usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file";
#  done
for file in target/debug/deps/*-*;
    do
        if [[ "$file" != *\.* ]];
        then mkdir -p "target/cov/$(basename $file)";
        /usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file";
        fi
    done
