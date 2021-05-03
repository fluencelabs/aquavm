import { AquamarineInterpreter, ParticleHandler } from '..';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const createTestIntepreter = async (handler: ParticleHandler) => {
    return AquamarineInterpreter.create(handler, vmPeerId, 'off', () => {});
};

describe('Tests', async () => {
    const interpreter = await createTestIntepreter(undefined);
    const testInvoke = (script, prevData, data): string => {
        prevData = new TextEncoder().encode(prevData);
        data = new TextEncoder().encode(data);
        return interpreter.invoke(vmPeerId, script, prevData, data);
    };

    it('should work', async () => {
        // arrange
        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = testInvoke(s, '[]', '[]');

        // assert
        expect(res).not.toBeUndefined();
    });
});
