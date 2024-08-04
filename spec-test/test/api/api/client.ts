/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/

import { getTest, postTest } from "./test";
import { createNested } from "./test/deep/and/nested";

class Client {
    BASE_PATH = "";

    API = {
		test: {
			postTest: postTest,
			getTest: getTest,
			deep: {
				and: {
					nested: {
						createNested: createNested
					}
				}
			}
		}
	}

    setBasePath(path: string) {
        this.BASE_PATH = path;
    }
}

const client = new Client();

export default client;

export type ApiResult<T, E> = {ok: true, value: T} | {ok: false, status: number,  error: E};
        