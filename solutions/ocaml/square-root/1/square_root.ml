let square_root n =
    let rec find sqrt i =
        if i * i = sqrt then i
        else if i * i > sqrt then 0
        else find sqrt (i+1)
    in
    find n 0
