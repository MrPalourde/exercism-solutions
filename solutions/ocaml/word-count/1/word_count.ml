open Base

let word_count phrase =
    let new_phrase = " " ^ phrase ^ " " in
    let new_phrase = String.lowercase new_phrase in
    let new_phrase =
        new_phrase
        |> String.substr_replace_all ~pattern:" '" ~with_:" "
        |> String.substr_replace_all ~pattern:"' " ~with_:" "
    in
    let words = String.split_on_chars new_phrase
                ~on:[ ' '; '\n'; '\t'; ','; ';'; ':'; '.'; '-'; '!'; '&'; '@'; '$'; '%'; '^' ]
                |> List.filter ~f:(fun w -> not (String.is_empty w))
    in
    List.fold_left
        ~init:(Map.empty (module String))
        ~f:(fun acc w ->
            Map.update acc w ~f:(function
            | None -> 1
            | Some n -> n + 1))
        words

