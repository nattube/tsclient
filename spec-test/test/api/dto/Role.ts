/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
import {type Option} from "./Option";
import {type Ability} from "./Ability";

type String = string;
type i32 = number;

export type Role = {
	id: i32;
	name: String;
	group_id: i32;
	abilities: Array<Ability>;
	meta_data: Option<any>
}