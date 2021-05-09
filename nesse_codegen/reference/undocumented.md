    Copied from nesdev wiki 2021-05-09

ALR #i ($4B ii; 2 cycles)
    Equivalent to AND #i then LSR A. Some sources call this "ASR"; we do not follow this out of confusion with the mnemonic for a pseudoinstruction that combines CMP #$80 (or ANC #$FF) then ROR. Note that ALR #$FE acts like LSR followed by CLC.

ANC #i ($0B ii, $2B ii; 2 cycles)
    Does AND #i, setting N and Z flags based on the result. Then it copies N (bit 7) to C. ANC #$FF could be useful for sign-extending, much like CMP #$80. ANC #$00 acts like LDA #$00 followed by CLC.

ARR #i ($6B ii; 2 cycles)
    Similar to AND #i then ROR A, except sets the flags differently. N and Z are normal, but C is bit 6 and V is bit 6 xor bit 5. A fast way to perform signed division by 4 is: CMP #$80; ARR #$FF; ROR. This can be extended to larger powers of two.

AXS #i ($CB ii; 2 cycles)
    Sets X to {(A AND X) - #value without borrow}, and updates NZC. One might use TXA AXS #-element_size to iterate through an array of structures or other elements larger than a byte, where the 6502 architecture usually prefers a structure of arrays. For example, TXA AXS #$FC could step to the next OAM entry or to the next APU channel, saving one byte and four cycles over four INXs. Also called SBX.

LAX (d,X) ($A3 dd; 6 cycles)
LAX d ($A7 dd; 3 cycles)
LAX a ($AF aa aa; 4 cycles)
LAX (d),Y ($B3 dd; 5 cycles)
LAX d,Y ($B7 dd; 4 cycles)
LAX a,Y ($BF aa aa; 4 cycles)
    Shortcut for LDA value then TAX. Saves a byte and two cycles and allows use of the X register with the (d),Y addressing mode. Notice that the immediate is missing; the opcode that would have been LAX is affected by line noise on the data bus. MOS 6502: even the bugs have bugs.

SAX (d,X) ($83 dd; 6 cycles)
SAX d ($87 dd; 3 cycles)
SAX a ($8F aa aa; 4 cycles)
SAX d,Y ($97 dd; 4 cycles)
    Stores the bitwise AND of A and X. As with STA and STX, no flags are affected.

    RMW instructions

    The read-modify-write instructions (INC, DEC, ASL, LSR, ROL, ROR) have few valid addressing modes, but these instructions have three more: (d,X), (d),Y, and a,Y. In some cases, it could be worth it to use these and ignore the side effect on the accumulator.

DCP (d,X) ($C3 dd; 8 cycles)
DCP d ($C7 dd; 5 cycles)
DCP a ($CF aa aa; 6 cycles)
DCP (d),Y ($D3 dd; 8 cycles)
DCP d,X ($D7 dd; 6 cycles)
DCP a,Y ($DB aa aa; 7 cycles)
DCP a,X ($DF aa aa; 7 cycles)
    Equivalent to DEC value then CMP value, except supporting more addressing modes. LDA #$FF followed by DCP can be used to check if the decrement underflows, which is useful for multi-byte decrements.

ISC (d,X) ($E3 dd; 8 cycles)
ISC d ($E7 dd; 5 cycles)
ISC a ($EF aa aa; 6 cycles)
ISC (d),Y ($F3 dd; 8 cycles)
ISC d,X ($F7 dd; 6 cycles)
ISC a,Y ($FB aa aa; 7 cycles)
ISC a,X ($FF aa aa; 7 cycles)
    Equivalent to INC value then SBC value, except supporting more addressing modes.

RLA (d,X) ($23 dd; 8 cycles)
RLA d ($27 dd; 5 cycles)
RLA a ($2F aa aa; 6 cycles)
RLA (d),Y ($33 dd; 8 cycles)
RLA d,X ($37 dd; 6 cycles)
RLA a,Y ($3B aa aa; 7 cycles)
RLA a,X ($3F aa aa; 7 cycles)
    Equivalent to ROL value then AND value, except supporting more addressing modes. LDA #$FF followed by RLA is an efficient way to rotate a variable while also loading it in A.

RRA (d,X) ($63 dd; 8 cycles)
RRA d ($67 dd; 5 cycles)
RRA a ($6F aa aa; 6 cycles)
RRA (d),Y ($73 dd; 8 cycles)
RRA d,X ($77 dd; 6 cycles)
RRA a,Y ($7B aa aa; 7 cycles)
RRA a,X ($7F aa aa; 7 cycles)
    Equivalent to ROR value then ADC value, except supporting more addressing modes. Essentially this computes A + value / 2, where value is 9-bit and the division is rounded up.

SLO (d,X) ($03 dd; 8 cycles)
SLO d ($07 dd; 5 cycles)
SLO a ($0F aa aa; 6 cycles)
SLO (d),Y ($13 dd; 8 cycles)
SLO d,X ($17 dd; 6 cycles)
SLO a,Y ($1B aa aa; 7 cycles)
SLO a,X ($1F aa aa; 7 cycles)
    Equivalent to ASL value then ORA value, except supporting more addressing modes. LDA #0 followed by SLO is an efficient way to shift a variable while also loading it in A.

SRE (d,X) ($43 dd; 8 cycles)
SRE d ($47 dd; 5 cycles)
SRE a ($4F aa aa; 6 cycles)
SRE (d),Y ($53 dd; 8 cycles)
SRE d,X ($57 dd; 6 cycles)
SRE a,Y ($5B aa aa; 7 cycles)
SRE a,X ($5F aa aa; 7 cycles)
    Equivalent to LSR value then EOR value, except supporting more addressing modes. LDA #0 followed by SRE is an efficient way to shift a variable while also loading it in A.

    Duplicated instructions

    Some instructions are equivalent to others. One possible use of these is for watermarking your binary if you want to make leaked executables traceable, such as copies of the ROM sent to testers or even individual cartridges sent to end users.

ADC #i ($69 ii, $E9 ii^$FF, $EB ii^$FF; 2 cycles)
SBC #i ($69 ii^$FF, $E9 ii, $EB ii; 2 cycles)
    $69 and $E9 are official; $EB is not. These three opcodes are nearly equivalent, except that $E9 and $EB add 255-i instead of i.

    NOPs

    Some instructions do nothing at all. These can be useful for wasting a small number of cycles, or for skipping past bytes to change the program's control flow. They can also be useful for padding or watermarking.

NOP ($1A, $3A, $5A, $7A, $DA, $EA, $FA; 2 cycles)
    The official NOP ($EA) and six unofficial NOPs do nothing. 

SKB #i ($80 ii, $82 ii, $89 ii, $C2 ii, $E2 ii; 2 cycles)
    These unofficial opcodes just read an immediate byte and skip it, like a different address mode of NOP. One of these even works almost the same way on 65C02, HuC6280, and 65C816: BIT #i ($89 ii), whose only difference from the 6502 is that it affects the NVZ flags like the other BIT instructions. Use this SKB if you want your code to be portable to Lynx, TG16, or Super NES. Puzznic uses $89, and Beauty and the Beast uses $80. Also called DOP, NOP (distinguished from the 1-byte encoding by the addressing mode).

IGN a ($0C aa aa; 4 cycles)
IGN a,X ($1C aa aa, $3C aa aa, $5C aa aa, $7C aa aa, $DC aa aa, $FC aa aa; 4 or 5 cycles)
IGN d ($04 dd, $44 dd, $64 dd; 3 cycles)
IGN d,X ($14 dd, $34 dd, $54 dd, $74 dd, $D4 dd, $F4 dd; 4 cycles)
    Reads from memory at the specified address and ignores the value. Affects no register nor flags. The absolute version can be used to increment PPUADDR or reset the PPUSTATUS latch as an alternative to BIT. The zero page version has no side effects.
    IGN d,X reads from both d and (d+X)&255. IGN a,X additionally reads from a+X-256 it crosses a page boundary (i.e. if ((a & 255) + X) > 255)
    Sometimes called TOP (triple-byte no-op), SKW (skip word), DOP (double-byte no-op), or SKB (skip byte).

CLD ($D8; 2 cycles)
CLV ($B8; 2 cycles)
SED ($F8; 2 cycles)
    These are official. CLD and SED control decimal mode, but on second-source 6502 CPUs without decimal mode such as the 2A03, they do almost nothing; their effect is visible only after a PHP or BRK. You can use them like NOP. And the V flag that CLV clears is rarely used; only ADC, BIT, SBC, the stack ops PLP and RTI, and the unofficial instructions ARR, ISC, and RRA affect it; the BVC and BVS instructions will check it.
