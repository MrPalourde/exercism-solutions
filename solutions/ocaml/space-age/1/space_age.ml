let convert_time time factor =
    (time /. factor) /. 31557600.

let age_on planet time =
    let time = float_of_int time in
    match planet with
    | "Earth" -> Ok(convert_time time 1.0)
    | "Mercury" -> Ok(convert_time time 0.2408467)
    | "Venus" -> Ok(convert_time time 0.61519726)
    | "Mars" -> Ok(convert_time time 1.8808158)
    | "Jupiter" -> Ok(convert_time time 11.862615)
    | "Saturn" -> Ok(convert_time time 29.447498)
    | "Uranus" -> Ok(convert_time time 84.016846)
    | "Neptune" -> Ok(convert_time time 164.79132)
    | _ -> Error "not a planet"
