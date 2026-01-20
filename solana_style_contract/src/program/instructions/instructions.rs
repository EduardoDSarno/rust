use solana_program::program_error::ProgramError;


pub enum ProgramInstruction{

    Initialize,
    Add,
    Subtract,
    Approve,    
}

impl ProgramInstruction{
    pub fn unpack(instruction_data: &[u8]) ->Result<Self,ProgramError>
    {
        let instruction = instruction_data
                          .get(0)
                          .ok_or(
                            ProgramError::InvalidInstructionData
                          )?;

        match instruction 
        {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Add),
            2 => Ok(Self::Subtract),
            3 => Ok(Self::Approve),
            _ => Err(ProgramError::InvalidInstructionData),
        }                  
    }

}