/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
import {type Group} from "./Group";


export type UserInfo = {
	id: number;
	name: string;
	refresh_pw: boolean;
	groups: Array<Group>;
	security_version: number
}