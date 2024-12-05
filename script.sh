for i in {02..25}
do
    # cargo new day$i
    cd day$i/src

    for j in {1..2}
    do
        # touch part$j.rs
        cat ../../template.rs > part$j.rs
        cat ../../main_template.rs > main.rs
    done

    cd ..
    touch input.txt
    cd ..
done
