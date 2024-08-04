/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/


export type Ok<T> = { Ok: T }


export type Err<E> = { Err: E }


export type Result<T, E> = Ok<T> | Err<E>