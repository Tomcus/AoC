let file_name = Array.get Sys.argv 1

let rec for_each_line file line_proc res_folder def_res =
    let line = try input_line file with End_of_file -> "" in
    if String.length line = 0 then
        def_res
    else
        res_folder (line_proc line) (for_each_line file line_proc res_folder def_res)

let for_each_line_from_file_input line_processor result_folder default_res =
    let file = open_in file_name in
    for_each_line file line_processor result_folder default_res

let rec parse_list str pattern pattern_with_delimiter =
    try
        Scanf.sscanf str pattern_with_delimiter (fun x rest -> x :: parse_list rest pattern pattern_with_delimiter)
    with Scanf.Scan_failure _ ->
        Scanf.sscanf str pattern (fun x -> [x])

let rec explode text =
    match text with
    | "" -> []
    | non_empty -> non_empty.[0] :: explode (String.sub non_empty 1 (String.length non_empty - 1))

let rec implode lst =
    match lst with
    | [] -> ""
    | c :: rest -> (String.make 1 c) ^ implode rest

