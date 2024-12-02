{
    inc = $2 > $1;
    diff = inc ? $2 - $1 : $1 - $2;
    good = diff > 0 && diff < 4;
    for(i=2; i < NF; i++) {
        diff = inc ? $(i+1) - $i : $i - $(i+1)
        good = good && diff > 0 && diff < 4
    }
    if (good) {
        count++
    }
}
END {print count}
