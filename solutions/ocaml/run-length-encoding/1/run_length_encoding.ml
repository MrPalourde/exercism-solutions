let is_digit s i =
  let c = String.get s i in
  c >= '0' && c <= '9'

let encode content =
    let encoded = ref "" in
    let current_char = ref ' ' in
    let current_times = ref 1 in
    let i = ref 0 in
    while !i < String.length content do
        if !i = 0 then 
            current_char := String.get content !i
        else begin
            if String.get content !i <> !current_char then begin
                if !current_times = 1 then
                    encoded := !encoded ^ String.make 1 (String.get content (!i-1))
                else begin
                    encoded := !encoded ^ string_of_int !current_times ^ String.make 1 (String.get content (!i-1));
                end;
                current_times := 1;
                current_char := String.get content !i;
            end else
                current_times := !current_times + 1;
        end;
        i := !i +1
    done;
    if String.length content <> 0 then
        if !current_times = 1 then
            encoded := !encoded ^ String.make 1 !current_char
        else begin
            encoded := !encoded ^ string_of_int !current_times ^ String.make 1 !current_char
        end;
    !encoded

let decode content =
    let decoded = ref "" in
    let i = ref 0 in
    let current_number = ref "" in
    while !i < String.length content do
        if is_digit content !i then
            current_number := !current_number ^ String.make 1 (String.get content !i)
        else begin
            if !current_number = "" then
                decoded := !decoded ^ String.make 1 (String.get content !i)
            else begin
                for _j = 0 to ((int_of_string !current_number) - 1) do
                    decoded := !decoded ^ String.make 1 (String.get content !i);
                done;
            end;
            current_number := "";
        end;
        i := !i + 1
    done;
    !decoded
