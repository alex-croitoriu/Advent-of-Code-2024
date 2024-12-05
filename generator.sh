# mkdir days
cd days

for i in {01..25}
do
    # cargo new day$i
    cd day$i/src

    for j in {1..2}
    do
        # touch part$j.rs
        cat ../../../templates/part.rs > part$j.rs
        # cat ../../../templates/main.rs > main.rs
    done

    cd ../..
done
