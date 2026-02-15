bst a    ; b = a & 0b0111
bxl 5    ; b = b ^ 0b0101
cdv b    ; c = a / 1>>b        this dictates the amount of numbers.
adv 3    ; a = a >> 3
bxc _    ; b = b ^ c
bxl 6    ; b = b ^ 0b0110
out b    ; print b
jnz 0    ; if(a == 0) goto 0

; for 16 numbers, `a` must be at least $1 << (3*15 + 1)$
; 2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0
