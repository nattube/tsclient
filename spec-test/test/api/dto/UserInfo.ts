/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
import {type Group} from "./Group";

type bool = boolean;
type i32 = number;
type String = string;

export type UserInfo = {
	id: i32;
	name: String;
	refresh_pw: bool;
	groups: Array<Group>;
	security_version: i32
}