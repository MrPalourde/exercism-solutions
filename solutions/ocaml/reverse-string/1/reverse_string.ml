let reverse_string word =
    word
    |> String.to_seq
    |> List.of_seq
    |> List.rev
    |> List.to_seq
    |> String.of_seq
