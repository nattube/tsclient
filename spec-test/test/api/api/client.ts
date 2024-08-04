/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/

import { createNested } from "./test/deep/and/nested";
import { getTest, postTest } from "./test";

class Client {
    BASE_PATH = "";

    API = {
		test: {
			deep: {
				and: {
					nested: {
						createNested: createNested
					}
				}
			},
			getTest: getTest,
			postTest: postTest
		}
	}

    setBasePath(path: string) {
        this.BASE_PATH = path;
    }
}

const client = new Client();

export default client;

export type ApiResult<T, E> = {ok: true, value: T} | {ok: false, status: number,  error: E};
        