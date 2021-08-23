import { AirInterpreter } from '..';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const createTestIntepreter = async () => {
    return AirInterpreter.create(vmPeerId, 'trace', (level, message) => {
        console.log(`level: ${level}, message=${message}`);
    });
};

describe('Tests', () => {
    it('should work', async () => {
        // arrange
        const i = await createTestIntepreter();

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = i.invoke(s, Buffer.from([]), Buffer.from([]), Buffer.from([]), Buffer.from([]));

        // assert
        expect(res).not.toBeUndefined();
    });
});
