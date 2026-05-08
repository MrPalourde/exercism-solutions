let digits n =
  let rec aux n acc =
    if n = 0 then acc
    else aux (n / 10) (n mod 10 :: acc)
  in
  if n = 0 then [0] else aux n []

let rec pow base exp =
    if exp = 0 then 1
    else base * pow base (exp - 1)

let validate candidate =
  let ds = digits candidate in
  let len = List.length ds in
  let total = List.fold_left (fun acc d -> acc + pow d len) 0 ds in
  total = candidate
