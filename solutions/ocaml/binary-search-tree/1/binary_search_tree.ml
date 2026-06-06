open Base

type bst =
    | Empty
    | Node of {
        right: bst;
        value: int;
        left: bst;
    }

let empty = Empty

let value = function
    | Empty -> Error "Empty tree has no value"
    | Node { value; _ } -> Ok value

let left = function
    | Empty -> Error "Empty tree has no left"
    | Node { left; _ } -> Ok left

let right = function
    | Empty -> Error "Empty tree has no right"
    | Node { right; _ } -> Ok right

let rec insert number tree =
    match tree with
    | Empty ->
        Node { right = Empty; value = number; left = Empty }
    | Node { right; value; left } ->
        if number <= value then
            Node { right; value; left = insert number left }
        else
            Node { right = insert number right; value; left }

let rec to_list = function
    | Empty -> []
    | Node { left; value; right } ->
        to_list left @ [value] @ to_list right
