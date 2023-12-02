let rec explode text =
    match text with
    | "" -> []
    | non_empty -> non_empty.[0] :: explode (String.sub non_empty 1 (String.length non_empty - 1))

let rec implode lst =
    match lst with
    | [] -> ""
    | c :: rest -> (String.make 1 c) ^ implode rest

