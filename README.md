# stalfos
A toy stack based VM written in rust.

Why is it called stalfos? I was watching someone play legend of zelda at the time. There was a stalfos on screen while I created the project.


## Architecture

stalfos is a stack based vm. all entries are stored nominally as 32 bit words. The type used is u32, but is converted to big-endian bytes when popped (u32 -> [u8,4]).

These 4-byte chunks are then converted to appropriate types (i32, f32) from the big-endian format. The result is reversed back to [u8,4]->u32 and stored.

individual bytes can be addressed by providing a pointer label and a number of bytes to offset. the allocation required loads each u32, chunks it into [u8,4] which are concatenated. The desired byte is then addressed.
 After this, the [u8,n] is chunked back to [u8,4]s and then converted to the required u32s to store again.
 


All jumps must be predefined with a JMP_DEF(string,address) before any other instructions. jmp instructions that point to an undefined label will panic.

labels are technically a nop at runtime, but are used to signify the start of a new function. providing a JMP_DEF label but not having that label appear at that location is fine.  The jump will occur to the listed location anway ( ie, JMP_DEF(<invalid>,999) -> JMP(<invalid>) will move the program to address 999, even if LABEL(<invalid>) does not occur at location 999. This is because labels are noops at runtime and simple serve as an empty address to start the execution at.


## Exception

EXCEPT_THROW and EXCEPT_CATCH are based on jmp_defs as above. upon triggering EXCEPT_THROW, the program counter will decrement until it reaches the last address that was jumped to. It will then jump to the location that was previous jumped from, and repeat. This occurrs until the operation found is an EXCEPT_CATCH. at this point, the program will JMP to the label defined on the EXCEPT_CATCH, which must be an existing JMP_DEF location.

currently, no memory is deallocated during this unwinding. This is a future TODO. 


## Returns

RET returns from the current jumped reference. This sets the program counter to the location it was originally jumped from. resuming execution on the instruction after the JMP, JMPe,JMPne,JMPs.
 
 Currently, no allocations are removed during this process. I am unsure if I will implemented this functionality. In normal execution, consider calling DEALLOC(ptr_id) on any allocations that do not outlive the 'function'
