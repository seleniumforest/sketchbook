(module
  ;; current gen is first $width bytes. next gen is second $width bytes
  (import "env" "memory" (memory 1))
  (import "console" "log" (func $log (param i32)))
  (import "console" "log_string" (func $log_string (param i32 i32)))
  (import "canvas" "fillRect" (func $fill_rect (param i32 i32 i32 i32 i32)))
  (global $empty_cell (export "empty_cell") (mut i32) (i32.const 0))
  (global $filled_cell (export "filled_cell") (mut i32) (i32.const 1))
  (global $cell_size (export "cell_size") (mut i32) (i32.const 5))
  (global $height (export "height") (mut i32) (i32.const 1000))
  (global $width (export "width") (mut i32) (i32.const 1000))
  (global $margin (export "margin") (mut i32) (i32.const 2))
  (func $init_first_row
    global.get $width
    i32.const 1
    i32.sub 
    i32.const 1
    i32.store8
  )

  ;; load n'th triplet
  (func $load_triplet (param i32) (result i32 i32 i32)
    local.get 0 
    i32.const 1 
    i32.sub
    i32.load8_u

    local.get 0
    i32.load8_u

    local.get 0 
    i32.const 1 
    i32.add
    i32.load8_u
  )

  (func $not (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.eq
    (if (result i32)
       (then 
         i32.const 0
         return 
       )
       (else 
         i32.const 1
         return 
       )
    )
  )

  ;; param L C R
  ;; C_new = (C and (not L or not R)) or (not C and R)
  (func $match_triplet (param i32 i32 i32) (result i32)
    (call $not (local.get 0))
    (call $not (local.get 2))
    i32.or
    local.get 1
    i32.and

    (call $not (local.get 1))
    local.get 2 
    i32.and

    i32.or
  )

  (func $next_generation
    (local $i i32)
    (local.set $i (i32.const 1))

    (loop $by_width
      local.get $i
      global.get $width
      i32.add
      (call $load_triplet (local.get $i))
      call $match_triplet
      i32.store8
                  
      ;; i = i + 1
      (local.set $i (i32.add (local.get $i) (i32.const 1)))

      ;; continue if i < width
      local.get $i
      global.get $width
      i32.lt_u
      br_if $by_width
    )
  )

  (func $shift_generations
    i32.const 0
    global.get $width
    global.get $width
    memory.copy
  )

  ;;param is offset_y 
  (func $draw_current_gen (param i32)
    (local $i i32)
    (local $offset_x i32)


    (local.set $i (i32.const 0))
    (local.set $offset_x (global.get $margin))

    (loop $by_width
      local.get $i 
      i32.load8_u
      i32.const 1
      i32.eq 
      (if (result i32)
        (then 
          global.get $filled_cell
        )
        (else 
          global.get $empty_cell
        )
      )
  
      ;;fill rect
      local.get $offset_x
      local.get 0
      global.get $cell_size
      global.get $cell_size
      call $fill_rect
  
       ;;offset_x += (cell_size + margin)
      local.get $offset_x
      (i32.add (global.get $cell_size) (global.get $margin))
      i32.add
      local.set $offset_x

      ;; i = i + 1
      (local.set $i (i32.add (local.get $i) (i32.const 1)))

      ;; continue if i < height
      (i32.lt_u (local.get $i) (global.get $width))
      br_if $by_width
    )
  )

  (func $main (export "main")
    (local $g i32)
    (local $offset_y i32)
    (local.set $g (i32.const 0))
    (local.set $offset_y (global.get $margin))

    call $init_first_row

    (loop $generations
      call $next_generation
      (call $draw_current_gen (local.get $offset_y))
      call $shift_generations

      ;;offset_y += (cell_size + margin)
      local.get $offset_y
      (i32.add (global.get $cell_size) (global.get $margin))
      i32.add
      local.set $offset_y

      (local.set $g (i32.add (local.get $g) (i32.const 1)))

      ;; continue if i < height
      (i32.lt_u (local.get $g) (global.get $height))
      br_if $generations
    )
  )
  (start $main)
)
