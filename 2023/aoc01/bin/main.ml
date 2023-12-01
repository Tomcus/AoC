let file_name = Array.get Sys.argv 1
let none_number = ' '

let rec explode text =
    match text with
    | "" -> []
    | non_empty -> non_empty.[0] :: explode (String.sub non_empty 1 (String.length non_empty - 1))

let rec process_line line first last =
    match line with
    | [] -> int_of_string (Printf.sprintf "%c%c" first last)
    | c :: rest -> match c with
        | '0'
        | '1'
        | '2'
        | '3'
        | '4'
        | '5'
        | '6'
        | '7'
        | '8'
        | '9' -> if first = none_number then
                process_line rest c c
            else
                process_line rest first c
        | _ -> process_line rest first last

let rec process_file file =
    let line = try input_line file with End_of_file -> "" in
    match String.length line with
    | 0 -> 0
    | _ -> process_line (explode line) none_number none_number + process_file file

let () =
    let file = open_in file_name in
    let res = process_file file in
    print_int res
    
