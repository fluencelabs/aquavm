import { AirInterpreter } from '..';
import { readFileSync } from 'fs';
import path from 'path';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const createTestInterpreter = async () => {
    const file = readFileSync(path.resolve(__dirname, './avm.wasm'));
    const module = await WebAssembly.compile(file);
    return AirInterpreter.create(module, 'off', (level, message) => {
        console.log(`level: ${level}, message=${message}`);
    });
};

const b = (s: string) => {
    return Buffer.from(s);
};

describe('Tests', () => {
    it('should work', async () => {
        // arrange
        const i = await createTestInterpreter();

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const params = { initPeerId: vmPeerId, currentPeerId: vmPeerId };
        const res = i.invoke(s, b(''), b(''), params, []);

        // assert
        console.log(res);
        expect(res).not.toBeUndefined();
    });
});
