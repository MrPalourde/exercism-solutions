let collatz_conjecture n = 
    if n < 1 then Error "Only positive integers are allowed"
    else (
        let rec collatz n i =
            if n = 1 then Ok i
            else (
                if n mod 2 = 0 then collatz (n/2) (i+1)
                else collatz (n*3+1) (i+1)
            )
        in
        collatz n 0
    )
