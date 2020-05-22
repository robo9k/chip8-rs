var searchIndex = JSON.parse('{\
"chip_8":{"doc":"Virtual machine for the CHIP-8 programming language","i":[[0,"errors","chip_8","Crate error types",null,null],[4,"Chip8Error","chip_8::errors","Error type for all errors in this crate",null,null],[13,"InvalidRegister","","Invalid register definition",0,null],[13,"UnknownInstruction","","Unknown instruction",0,null],[6,"Result","","Result alias",null,null],[0,"instructions","chip_8","Machine language and byte code instructions",null,null],[3,"Addr","chip_8::instructions","Absolute memory address",null,null],[3,"Nibble","","Hex digit",null,null],[4,"VRegister","","General purpose register",null,null],[13,"V0","","",1,null],[13,"V1","","",1,null],[13,"V2","","",1,null],[13,"V3","","",1,null],[13,"V4","","",1,null],[13,"V5","","",1,null],[13,"V6","","",1,null],[13,"V7","","",1,null],[13,"V8","","",1,null],[13,"V9","","",1,null],[13,"VA","","",1,null],[13,"VB","","",1,null],[13,"VC","","",1,null],[13,"VD","","",1,null],[13,"VE","","",1,null],[13,"VF","","",1,null],[4,"Instruction","","Byte code instruction",null,null],[13,"Sys","","Jumps to machine routine at `Addr`",2,null],[13,"Clear","","Clears the display",2,null],[13,"Return","","Returns from a subroutine",2,null],[13,"Jump","","Jumps to `Addr`",2,null],[13,"Call","","Calls subroutine at `Addr`",2,null],[13,"SkipEqualOperand","","Skips next instruction if `Vx` equals `byte`",2,null],[13,"SkipNotEqualOperand","","Skips next instruction if `Vx` is not equal to `byte`",2,null],[13,"SkipEqual","","Skips next instruction if `Vy` is equal to `Vy`",2,null],[13,"LoadOperand","","Loads `byte` into `Vx`",2,null],[13,"AddOperand","","Adds `byte` to `Vx`, then stores it in `Vx`",2,null],[13,"Load","","Loads `Vy` into `Vx`",2,null],[13,"Or","","Sets `Vx` to `Vx OR Vy`",2,null],[13,"And","","Sets `Vx` to `Vx AND Vy`",2,null],[13,"XOr","","Sets `Vx` to `Vx XOR Vy`",2,null],[13,"Add","","Sets `Vx` to `Vx + Vy`, `VF` to carry",2,null],[13,"Sub","","Sets `Vx` to `Vx - Vy`, `VF` to not borrow",2,null],[13,"ShiftRight","","Sets `Vx` to `Vy SHR 1`",2,null],[13,"SubNegated","","Sets `Vx` to `Vy - Vx`, `VF` to not borrow",2,null],[13,"ShiftLeft","","Sets `Vx` to `Vy SHL 1`",2,null],[13,"SkipNotEqual","","Skips next instruction if `Vx` is not equal to `Vy`",2,null],[13,"LoadI","","Loads `Addr` into register `I`",2,null],[13,"LongJump","","Jumps to `Addr + V0`",2,null],[13,"Random","","Sets `Vx` to random number AND `kk`",2,null],[13,"Draw","","Read `n` bytes of memory from address `I`, draw it at `Vx`…",2,null],[13,"SkipKeyPressed","","Skip next instruction if key `Vx` is pressed",2,null],[13,"SkipKeyNotPressed","","Skip next instruction if key `Vx` is not pressed",2,null],[13,"LoadRegisterDelayTimer","","Set `Vx` to delay timer value",2,null],[13,"LoadKey","","Wait for key press and store it in `Vx`",2,null],[13,"LoadDelayTimerRegister","","Set delay timer to `Vx`",2,null],[13,"LoadSoundTimerRegister","","Set sound timer to `Vx`",2,null],[13,"AddI","","Add `Vx` to `I`",2,null],[13,"LoadSprite","","Set `I` to the address of the sprite `Vx`",2,null],[13,"LoadBinaryCodedDecimal","","Store binary-coded decimal (BCD) at `I`, `I`+1 and `I`+2",2,null],[13,"LoadMemoryRegisters","","Store registers `V0`..`Vx` in memory at `I`",2,null],[13,"LoadRegistersMemory","","Read registers `V0`..`Vx` from memory at `I`",2,null],[6,"Vx","","First register in an instruction",null,null],[6,"Vy","","Second register in an instruction",null,null],[6,"Byte","","A byte",null,null],[11,"iter_to","","Returns an `Iterator` from `V0` up to including…",1,[[["vregister",4]]]],[11,"decode","","Decodes raw `bits` into a valid `Instruction`",2,[[],["result",6]]],[0,"vm","chip_8","Virtual machine",null,null],[3,"VM","chip_8::vm","Virtual machine",null,null],[11,"new","","Creates a new instance",3,[[]]],[11,"from","chip_8::errors","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_string","","",0,[[],["string",3]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","chip_8::instructions","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","chip_8::vm","",3,[[]]],[11,"into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","chip_8::instructions","",4,[[]]],[11,"from","","",5,[[]]],[11,"clone","","",1,[[],["vregister",4]]],[11,"clone","","",4,[[],["addr",3]]],[11,"clone","","",5,[[],["nibble",3]]],[11,"clone","","",2,[[],["instruction",4]]],[11,"eq","chip_8::errors","",0,[[["chip8error",4]]]],[11,"ne","","",0,[[["chip8error",4]]]],[11,"eq","chip_8::instructions","",1,[[["vregister",4]]]],[11,"eq","","",4,[[["addr",3]]]],[11,"ne","","",4,[[["addr",3]]]],[11,"eq","","",5,[[["nibble",3]]]],[11,"ne","","",5,[[["nibble",3]]]],[11,"eq","","",2,[[["instruction",4]]]],[11,"ne","","",2,[[["instruction",4]]]],[11,"fmt","chip_8::errors","",0,[[["formatter",3]],["result",6]]],[11,"fmt","chip_8::instructions","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","chip_8::vm","",3,[[["formatter",3]],["result",6]]],[11,"fmt","chip_8::errors","",0,[[["formatter",3]],["result",6]]],[11,"try_from","chip_8::instructions","",1,[[],["result",4]]]],"p":[[4,"Chip8Error"],[4,"VRegister"],[4,"Instruction"],[3,"VM"],[3,"Addr"],[3,"Nibble"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);