let raindrop n =
    let s = "" in
    let s = if n mod 3 = 0 then
                s ^ "Pling"
            else
                s
    in
    let s = if n mod 5 = 0 then
                s ^ "Plang"
            else
                s
    in
    let s = if n mod 7 = 0 then
                s ^ "Plong"
            else 
                s
    in
    if String.length s = 0 then
        string_of_int n
    else
        s
