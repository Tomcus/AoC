let file_name = Array.get Sys.argv 1

exception Imposible of string

let rec split_impl separator remaining current =
    match remaining with
    | [] -> (List.rev current, [])
    | c :: rest ->
        if c = separator then
            (List.rev current), rest 
        else
            split_impl separator rest (c :: current)

let split separator input =
    split_impl separator input []

let validate_color_and_val num color =
    match color with
    | "red" -> [num; 1; 1] 
    | "green" -> [1; num; 1]
    | "blue" -> [1; 1; num]
    | _ -> raise (Imposible "validate_color_and_val")

let rec valid_pull pull =
    match split ',' pull with
    | (number_and_color, []) -> 
        Scanf.sscanf (Utils.implode number_and_color) " %i %s" validate_color_and_val
    | (number_and_color, rest) ->
        List.map2 Int.max (Scanf.sscanf (Utils.implode number_and_color) " %i %s" validate_color_and_val) (valid_pull rest)

let rec valid_pulls pulls =
    match split ';' pulls with
    | (pull, []) -> valid_pull pull
    | (pull, rest) -> List.map2 Int.max (valid_pull pull) (valid_pulls rest)

let valid pulls = 
    valid_pulls pulls

let process_line line =
    match split ':' line with
    | (_, pulls) ->
        List.fold_left ( * ) 1 (valid pulls)

let rec process_file file =
    let line = try input_line file with End_of_file -> "" in
    match Utils.explode line with
    | [] -> 0
    | 'G' :: 'a' :: 'm' :: 'e' :: ' ' :: rest ->
        let res = process_line rest in
        Printf.printf "%s -> %i\n" line res;
        res + process_file file
    | _ -> raise (Imposible "process_file")

let () =
    let res = process_file (open_in file_name) in
    Printf.printf "%i\n" res
