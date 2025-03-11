
const postgresServer = new azure.dbforpostgresql.FlexibleServer("myPostgres", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    administratorLogin: "adminUser",
    administratorLoginPassword: "strongPassword123!",
    version: "13", // PostgreSQL version
    sku: {
        name: "Standard_D2s_v3",
        tier: "GeneralPurpose",
        capacity: 2,
    },
    storage: {
        storageSizeGb: 32,
    },
});

export const postgresHost = postgresServer.fqdn;
export const postgresAdmin = postgresServer.administratorLogin;
