#[derive(Debug)]
pub struct Data {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

#[derive(Debug)]
pub struct Ort {
    pub a: u32,
    pub data: u32
}

#[derive(Debug)]
pub enum Operator {
    ConditionalMove(Data),
    ArrayIndex(Data),
    ArrayAmendment(Data),
    Addition(Data),
    Multiplication(Data),
    Division(Data),
    NotAnd(Data),
    Halt,
    Allocation(Data),
    Abandonment(Data),
    Output(Data),
    Input(Data),
    LoadProgram(Data),
    Orthography(Ort),
}