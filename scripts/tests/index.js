// Import
import { ApiPromise, WsProvider, HttpProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

async function main() {
    // Construct
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    //const httpProvider = new HttpProvider('http://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    //const now = await api.query.timestamp.now();
    // Do something
    //console.log(now);
    // query on chain 
    // let alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    // const account = await api.query.system.account(alice);
    // console.log(account.toHuman());

    // thực hiện transaction 
    const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    // Construct the keyring after the API (crypto has an async init)
    const keyring = new Keyring({ type: 'sr25519' });

    // Add Alice to our keyring with a hard-derivation path (empty phrase, so uses dev)
    const alice = keyring.addFromUri('//Alice');
    const dung = keyring.addFromMnemonic('tube beyond side ivory wisdom apple intact slice pool bottom security heavy');
    console.log(alice);

    // Create a extrinsic, transferring 12345 units to Bob
    const transfer = api.tx.balances.transfer(BOB, 12345);

    // Sign and send the transaction using our account
    const hash = await transfer.signAndSend(alice);

    console.log('Transfer sent with hash', hash.toHex());

    const valueBefore = await api.query.templateModule.something();
    console.log(valueBefore.toHuman().toString());

    // const decrease = api.tx.templateModule.decrease();
    // const hashDec = await decrease.signAndSend(alice);


    //console.log('Dec sent with hash', hashDec.toHex());

    api.tx.templateModule
    .decrease()
    .signAndSend(alice, ({ events = [], status }) => {
      console.log('Transaction status:', status.type);

      if (status.isInBlock) {
        console.log('Included at block hash', status.asInBlock.toHex());
        console.log('Events:');

        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
        });

      } else if (status.isFinalized) {
        console.log('Finalized block hash', status.asFinalized.toHex());
        
      }
    });


    const valueAfter = await api.query.templateModule.something();
    console.log(valueAfter.toHuman().toString());


}

main()