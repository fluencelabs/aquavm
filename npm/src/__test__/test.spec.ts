import { AquamarineInterpreter, ParticleHandler } from '..';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const createTestIntepreter = async (handler: ParticleHandler) => {
    return AquamarineInterpreter.create(handler, vmPeerId, 'trace', (level, message) => {
        switch (level) {
            case 0:
            case 1:
            case 2:
                console.log(message);
            case 3:
                console.warn(message);
            case 4:
                console.error(message);
        }
    });
};

const testInvoke = (interpreter, script, prevData, data): string => {
    prevData = Buffer.from(prevData);
    data = Buffer.from(data);
    return interpreter.invoke(vmPeerId, script, prevData, data);
};

describe('Tests', () => {
    it('should work', async () => {
        // arrange
        const i = await createTestIntepreter(() => {
            return {
                ret_code: 0,
                result: '{}',
            };
        });

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = testInvoke(i, s, [], []);

        // assert
        expect(res).not.toBeUndefined();
    });
});
