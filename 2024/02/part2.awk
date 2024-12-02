function safe(fields, num_fields) {
    inc = fields[2] > fields[1];
    diff = inc ? fields[2] - fields[1] : fields[1] - fields[2];
    good = diff > 0 && diff < 4;
    for(i = 2; i < num_fields; i++) {
        diff = inc ? fields[i+1] - fields[i] : fields[i] - fields[i+1];
        good = good && diff > 0 && diff < 4;
    }
    return good
}

function skip(src, dest, num_fields, s) {
    j = 1
    for (i = 1; i <= num_fields; i++) {
        if (i != s) {
            dest[j] = src[i];
            j++
        }
    }
}

{
    num_fields = NF;
    for (i = 1; i <= NF; i++) {
        fields[i] = $i;
    }
    good = safe(fields, num_fields);
    for (k = 1; k <= NF; k++) {
        skip(fields, dest, num_fields, k);

        good = good || safe(dest, num_fields - 1);
    }
    if (good) {
        count++;
    }
}

END {
    print count;
}
