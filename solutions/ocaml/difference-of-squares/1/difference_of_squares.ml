let square_of_sum n = 
    let rec sum y =
        if y = 1 then 1
        else y + sum (y-1)
    in
    let total = sum n in
    total * total

let rec sum_of_squares n =
    if n = 1 then 1
    else n*n + sum_of_squares (n-1)

let difference_of_squares n =
    let sum = sum_of_squares n in
    let square = square_of_sum n in
    if sum > square then sum - square
    else square - sum
