/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
import {type Option} from "./Option";
import {type Ability} from "./Ability";


export type Role = {
	id: number;
	name: string;
	group_id: number;
	abilities: Array<Ability>;
	meta_data: Option
}