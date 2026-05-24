let is_triangle x y z =
    if (x+y >= z) && (y+z >= x) && (x+z >= y) then
        if x <> 0 && y <> 0 && z <> 0 then
            true
        else
            false
    else
        false

let is_equilateral x y z =
    if x = y then
        if x = z then
            true && is_triangle x y z
        else
            false
    else
        false

let is_isosceles x y z =
    if x = y then
        true && is_triangle x y z
    else if x = z then
        true && is_triangle x y z
    else if y = z then
        true && is_triangle x y z
    else false && is_triangle x y z

let is_scalene x y z =
    not(is_isosceles x y z) && is_triangle x y z
