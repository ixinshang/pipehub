CREATE TABLE TENANTS
(
    ID           BIGSERIAL PRIMARY KEY,
    APP_ID       BIGINT  NOT NULL,
    GITHUB_LOGIN VARCHAR NOT NULL,
    GITHUB_ID    BIGINT  NOT NULL
);

CREATE TABLE WECHAT_WORKS
(
    ID        BIGSERIAL PRIMARY KEY,
    TENANT_ID BIGINT  NOT NULL,
    CORP_ID   VARCHAR NOT NULL,
    AGENT_ID  BIGINT  NOT NULL,
    SECRET    VARCHAR NOT NULL
)