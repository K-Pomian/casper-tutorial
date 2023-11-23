import { CasperServiceByJsonRPC } from "casper-js-sdk";
import { NODE_URL } from "./static";
import * as log4js from "log4js";

const main = async () => {
  let currentTime = new Date();
  log4js.configure({
    appenders: {
      default: {
        type: "stdout",
      },
      blockInfo: {
        type: "file",
        filename: "logs/".concat(currentTime.toISOString().concat(".log")),
      },
      status: {
        type: "console",
      },
    },
    categories: {
      default: { appenders: ["default"], level: "trace" },
      blockInfo: { appenders: ["blockInfo"], level: "info" },
      status: { appenders: ["status"], level: "info" },
    },
  });

  const blockInfoLogger = log4js.getLogger("blockInfo");

  const rpcServices = new CasperServiceByJsonRPC(NODE_URL);
  const latestBlockInfo = await rpcServices.getLatestBlockInfo();
  blockInfoLogger.info(latestBlockInfo);

  const statusLogger = log4js.getLogger("status");
  const status = await rpcServices.getStatus();
  statusLogger.info(status);
};

main();
