/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
import {type Group} from "./Group";

type i32 = number;
type String = string;
type bool = boolean;

export type UserInfo = {
	id: i32;
	name: String;
	refresh_pw: bool;
	groups: Array<Group>;
	security_version: i32
}