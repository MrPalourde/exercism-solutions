let egg_count number =
  let rec aux x acc =
    if x = 0 then acc
    else aux (x land (x - 1)) (acc + 1)
  in
  aux number 0
