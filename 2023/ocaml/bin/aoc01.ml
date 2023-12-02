let file_name = Array.get Sys.argv 1
let none_number = ' '

let rec explode text =
    match text with
    | "" -> []
    | non_empty -> non_empty.[0] :: explode (String.sub non_empty 1 (String.length non_empty - 1))

let rec process_line line first last =
    let helper rest first peek =
        if first = none_number then
            process_line rest peek peek
        else
            process_line rest first peek
    in
    match line with
    | [] -> int_of_string (Printf.sprintf "%c%c" first last)
    | 'o' :: 'n' :: 'e' :: rest -> helper rest first '1'
    | 't' :: 'w' :: 'o' :: rest -> helper rest first '2'
    | 't' :: 'h' :: 'r' :: 'e' :: 'e' :: rest -> helper rest first '3'
    | 'f' :: 'o' :: 'u' :: 'r' :: rest -> helper rest first '4'
    | 'f' :: 'i' :: 'v' :: 'e' :: rest -> helper rest first '5'
    | 's' :: 'i' :: 'x' :: rest -> helper rest first '6'
    | 's' :: 'e' :: 'v' :: 'e' :: 'n' :: rest -> helper rest first '7'
    | 'e' :: 'i' :: 'g' :: 'h' :: 't' :: rest -> helper rest first '8'
    | 'n' :: 'i' :: 'n' :: 'e' :: rest -> helper rest first '9'
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
        | '9' -> helper rest first c 
        | _ -> process_line rest first last

let rec process_file file =
    let line = try input_line file with End_of_file -> "" in
    match String.length line with
    | 0 -> 0
    | _ -> 
        let line_res = process_line (explode line) none_number none_number in
        line_res + process_file file

let () =
    let file = open_in file_name in
    let res = process_file file in
    Printf.printf "%i\n" res
