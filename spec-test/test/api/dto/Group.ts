/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
import {type Option} from "./Option";
import {type Role} from "./Role";

type i32 = number;
type String = string;

export type Group = {
	id: i32;
	name: String;
	roles: Array<Role>;
	meta_data: Option<any>
}