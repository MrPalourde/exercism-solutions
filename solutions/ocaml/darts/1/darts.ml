let score (x: float) (y: float): int =
    let d = sqrt (x *. x +. y *. y) in
    match d with
    | d when d <= 1. -> 10
    | d when d <= 5. -> 5
    | d when d <= 10. -> 1
    | _ -> 0
