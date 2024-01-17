#[test]
fn loop_strength_reduction() -> Result<(), egglog::Error> {
    let build = "
        (let outer-id (Id (i64-fresh!)))
        (let loop-id (Id (i64-fresh!)))
        (let input-list
                (Cons (Num outer-id 0) ; a
                (Cons (Num outer-id 0) ; i
                (Cons (Num outer-id 3) (Nil)))) ; c
        )
        (let inputs
            (
                All
                (Parallel)
                input-list
            )
        )
        (let pred
            (LessThan (Get (Arg loop-id) 1) (Num loop-id 4))
        )
        (let output-list
            (Cons (Mul (Get (Arg loop-id) 2) (Get (Arg loop-id) 1)) ; i * c
            (Cons (Add (Get (Arg loop-id) 1) (Num loop-id 1)) ; i += 1
            (Cons (Get (Arg loop-id) 2) (Nil))))
        )
        (let loop
            (
                Loop
                loop-id
                inputs
                (All
                    (Sequential)
                    (Pair
                        pred
                        (All
                            (Parallel)
                            output-list
                        )
                    )
                )
            )
        )
    ";

    let check = "
        (check (
            =
            loop
            (Loop
                whatever-id
                (All
                    (Parallel)
                    (Cons (Num outer-id 0) ; a
                    (Cons (Num outer-id 0) ; i
                    (Cons (Num outer-id 3) ; c
                    (Cons
                        (Mul (Num outer-id 3) (Num outer-id 0))
                        (Nil))))))
                (All
                    (Sequential)
                    (Cons
                        (LessThan (Get (Arg whatever-id) 1) (Num whatever-id 4))
                        (Cons (All
                            (Parallel)
                            (Cons (Get (Arg whatever-id) 3) ; i * c => d
                            (Cons (Add (Get (Arg whatever-id) 1) (Num whatever-id 1)) ; i += 1
                            (Cons (Get (Arg whatever-id) 2)
                            (Cons 
                                (Add (Get (Arg whatever-id) 3) (Mul (Get (Arg whatever-id) 2)
                                    (Num whatever-id 1)
                                  ))
                            (Nil)))))) (Nil))
                            ))
            )
    ))
    ";

    crate::run_test(build, check)
}
