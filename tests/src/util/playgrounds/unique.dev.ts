// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// SPDX-License-Identifier: Apache-2.0

import {mnemonicGenerate} from '@polkadot/util-crypto';
import {UniqueHelper} from './unique';
import {ApiPromise, WsProvider} from '@polkadot/api';
import * as defs from '../../interfaces/definitions';
import {IKeyringPair} from '@polkadot/types/types';


export class SilentLogger {
  log(_msg: any, _level: any): void { }
  level = {
    ERROR: 'ERROR' as const,
    WARNING: 'WARNING' as const,
    INFO: 'INFO' as const,
  };
}


export class SilentConsole {
  // TODO: Remove, this is temporary: Filter unneeded API output
  // (Jaco promised it will be removed in the next version)
  consoleErr: any;
  consoleLog: any;
  consoleWarn: any;

  constructor() {
    this.consoleErr = console.error;
    this.consoleLog = console.log;
    this.consoleWarn = console.warn;
  }

  enable() {  
    const outFn = (printer: any) => (...args: any[]) => {
      for (const arg of args) {
        if (typeof arg !== 'string')
          continue;
        if (arg.includes('1000:: Normal connection closure') || arg.includes('Not decorating unknown runtime apis: UniqueApi/2, RmrkApi/1') || arg.includes('RPC methods not decorated:') || arg === 'Normal connection closure')
          return;
      }
      printer(...args);
    };
  
    console.error = outFn(this.consoleErr.bind(console));
    console.log = outFn(this.consoleLog.bind(console));
    console.warn = outFn(this.consoleWarn.bind(console));
  }

  disable() {
    console.error = this.consoleErr;
    console.log = this.consoleLog;
    console.warn = this.consoleWarn;
  }
}


export class DevUniqueHelper extends UniqueHelper {
  /**
   * Arrange methods for tests
   */
  arrange: ArrangeGroup;

  constructor(logger: { log: (msg: any, level: any) => void, level: any }) {
    super(logger);
    this.arrange = new ArrangeGroup(this);
  }

  async connect(wsEndpoint: string, _listeners?: any): Promise<void> {
    const wsProvider = new WsProvider(wsEndpoint);
    this.api = new ApiPromise({
      provider: wsProvider,
      signedExtensions: {
        ContractHelpers: {
          extrinsic: {},
          payload: {},
        },
        FakeTransactionFinalizer: {
          extrinsic: {},
          payload: {},
        },
      },
      rpc: {
        unique: defs.unique.rpc,
        rmrk: defs.rmrk.rpc,
        eth: {
          feeHistory: {
            description: 'Dummy',
            params: [],
            type: 'u8',
          },
          maxPriorityFeePerGas: {
            description: 'Dummy',
            params: [],
            type: 'u8',
          },
        },
      },
    });
    await this.api.isReadyOrError;
    this.network = await UniqueHelper.detectNetwork(this.api);
  }
}

class ArrangeGroup {
  helper: UniqueHelper;

  constructor(helper: UniqueHelper) {
    this.helper = helper;
  }

  /**
   * Generates accounts with the specified UNQ token balance 
   * @param balances balances for generated accounts. Each balance will be multiplied by the token nominal.
   * @param donor donor account for balances
   * @returns array of newly created accounts
   * @example const [acc1, acc2, acc3] = await createAccounts([0n, 10n, 20n], donor); 
   */
  createAccounts = async (balances: bigint[], donor: IKeyringPair): Promise<IKeyringPair[]> => {
    let nonce = await this.helper.chain.getNonce(donor.address);
    const ss58Format = this.helper.chain.getChainProperties().ss58Format;
    const tokenNominal = this.helper.balance.getOneTokenNominal();
    const transactions = [];
    const accounts: IKeyringPair[] = [];
    for (const balance of balances) {
      const recipient = this.helper.util.fromSeed(mnemonicGenerate(), ss58Format);
      accounts.push(recipient);
      if (balance !== 0n) {
        const tx = this.helper.constructApiCall('api.tx.balances.transfer', [{Id: recipient.address}, balance * tokenNominal]);
        transactions.push(this.helper.signTransaction(donor, tx, 'account generation', {nonce}));
        nonce++;
      }
    }

    await Promise.all(transactions).catch(_e => {});
    
    //#region TODO remove this region, when nonce problem will be solved
    const checkBalances = async () => {
      let isSuccess = true;
      for (let i = 0; i < balances.length; i++) {
        const balance = await this.helper.balance.getSubstrate(accounts[i].address);
        if (balance !== balances[i] * tokenNominal) {
          isSuccess = false;
          break;
        }
      }
      return isSuccess;
    };

    let accountsCreated = false;
    // checkBalances retry up to 5 blocks
    for (let index = 0; index < 5; index++) {
      accountsCreated = await checkBalances();
      if(accountsCreated) break;
      await this.waitNewBlocks(1);
    }

    if (!accountsCreated) throw Error('Accounts generation failed');
    //#endregion

    return accounts;
  };
 
  /**
   * Wait for specified bnumber of blocks
   * @param blocksCount number of blocks to wait
   * @returns 
   */
  async waitNewBlocks(blocksCount = 1): Promise<void> {
    // eslint-disable-next-line no-async-promise-executor
    const promise = new Promise<void>(async (resolve) => {
      const unsubscribe = await this.helper.api!.rpc.chain.subscribeNewHeads(() => {
        if (blocksCount > 0) {
          blocksCount--;
        } else {
          unsubscribe();
          resolve();
        }
      });
    });
    return promise;
  }
}