module IntMap = Map.Make(Int64);;

exception Fatal of string;;

let rec powish_2 numb =
    match numb with
    | 0 -> 0
    | 1 -> 1
    | _ -> 2 * powish_2 (numb - 1)

let rec is_winning winning num =
    match winning with
    | [] -> false
    | first :: rest ->
        if num = first then
            true
        else
            is_winning rest num

let rec calculate_matching winning drawn =
    match drawn with
    | [] -> 0
    | first :: rest ->
        if is_winning winning first then
            1 + calculate_matching winning rest
        else
            calculate_matching winning rest

let rec iota start_num count =
    if count == 0 then
        []
    else
        Int64.of_int (start_num + 1) :: iota (start_num + 1) (count - 1)

let rec add_one_to_map map list_of_items =
    match list_of_items with
    | [] -> map
    | x :: rest ->
        match (IntMap.find_opt x map) with
        | Some i -> add_one_to_map (map |> IntMap.update x (Option.map (fun _ -> i + 1))) rest
        | None -> add_one_to_map (map |> IntMap.add x 1) rest

let stoi str =
    let str = String.trim str in
    int_of_string_opt str

let rec calculate_score_for_line line =
    let first_split = String.split_on_char ':' line in
    let card_number = Scanf.sscanf (List.nth first_split 0) "Card %d" (fun x -> x) in
    let data = List.nth first_split 1 in
    let second_split = String.split_on_char '|' (String.trim data) in
    let winning = List.filter_map stoi (String.split_on_char ' ' (List.nth second_split 0)) in
    let drawn = List.filter_map stoi (String.split_on_char ' ' (List.nth second_split 1)) in
    let count = calculate_matching winning drawn in
    let new_cards = iota card_number count in
    (card_number, new_cards)

let () =
    let res = Utils.fold_line_from_file_input calculate_score_for_line (fun data map ->
        match data with
        | (card_number, new_cards) ->
            match IntMap.find_opt (Int64.of_int card_number) map with
            | None -> add_one_to_map map new_cards
            | Some x -> List.fold_left (fun map _ -> add_one_to_map map new_cards) map (iota 0 (x + 1))
    ) IntMap.empty in
    Printf.printf "Result: %d\n" res
