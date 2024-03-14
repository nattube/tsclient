/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/

import { type UserInfo } from "../dto/UserInfo";
import { type Test } from "../dto/Test";
import { type Test3 } from "../dto/Test3";
import __client__, {type ApiResult} from "./client"

 
export async function getTest_RAW(): Promise<Response> {
    const headers = new Headers({
        
    });

    const __body = null;

    const __queryString = "";

    let __result = await fetch(`${__client__.BASE_PATH}/api/test${__queryString}`, {
        method: 'GET',
        headers: headers,
        body: __body
    });

    return __result
}



export async function getTest(): Promise<ApiResult<UserInfo, any>> {
    let __result = await getTest_RAW();

    if(!__result.ok) {
        let error = await __result.json();
        return {
            ok: false, 
            status: __result.status,  
            error
        }
    } else {
        let value = await __result.json();
        return {
            ok: true,
            value
        }
    }
}



export async function postTest_RAW(test3: Test3, test: Test): Promise<Response> {
    const headers = new Headers({
        'Content-Type': 'application/json'
    });

    const __body = JSON.stringify(test);

    
    const __params = new URLSearchParams();

	if(test3?.field1 != null) { test3.field1.forEach(val => __params.append('field1', val.toString())); }
	if(test3?.field2 != null) { __params.append('field2', test3.field2.toString()) }
	if(test3?.field3 != null) { __params.append('field3', test3.field3.toString()) }
	if(test3?.field4 != null) { __params.append('field4', test3.field4.toString()) }

    const __queryString = "?" + __params.toString();


    let __result = await fetch(`${__client__.BASE_PATH}/api/test${__queryString}`, {
        method: 'POST',
        headers: headers,
        body: __body
    });

    return __result
}



export async function postTest(test3: Test3, test: Test): Promise<ApiResult<Test, any>> {
    let __result = await postTest_RAW(test3, test);

    if(!__result.ok) {
        let error = await __result.json();
        return {
            ok: false, 
            status: __result.status,  
            error
        }
    } else {
        let value = await __result.json();
        return {
            ok: true,
            value
        }
    }
}
