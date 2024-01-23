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
			deep: {
				and: {
					nested: {
						createNested: createNested
					}
				}
			},
			postTest: postTest,
			getTest: getTest
		}
	}

    setBasePath(path: string) {
        this.BASE_PATH = path;
    }
}

const client = new Client();

export default client;

export type ApiResult<T, E> = T | {isError: true, status_code: number,  error: E};
        