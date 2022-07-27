-- Your SQL goes here
INSERT INTO "settings" ("id", "key", "value", "desc") VALUES (1, "gateway", "https://ipfs.io", "默认网关");
INSERT INTO "settings" ("id", "key", "value", "desc") VALUES (2, "GITHUB_TOKEN", "", "github的token用来触发actions");
INSERT INTO "settings" ("id", "key", "value", "desc") VALUES (3, "GITHUB_ACTIONS_URL", "", "github的actions网址");
INSERT INTO "settings" ("id", "key", "value", "desc") VALUES (4, "WEB3_STORAGE_TOKEN", "", "https://web3.storage的token");
INSERT INTO "settings" ("id", "key", "value", "desc") VALUES (5, "DB_CONNECT", "", "mongodb的链接字符串，可以从mongodb.com获得");
INSERT INTO "settings" ("id", "key", "value", "desc") VALUES (6, "SYNC_TOPIC", "", "上传成功订阅主题");