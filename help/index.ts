// @ts-ignore
import * as pulumi from "@pulumi/pulumi";
// @ts-ignore
import * as azure from "@pulumi/azure-native";

// Configuration variables (optional customization via Pulumi config)
const config = new pulumi.Config();
const location = config.get("location") || "eastus2"; // Default location
const resourceGroupName = config.get("resourceGroupName") || "test-rg";

// Create an Azure Resource Group
const resourceGroup = new azure.resources.ResourceGroup("example-rg", {
    resourceGroupName: resourceGroupName,
    location: location,
});

// Create an Azure Storage Account
const storageAccount = new azure.storage.StorageAccount("examplestorage", {
    resourceGroupName: resourceGroup.name,
    location: resourceGroup.location,
    sku: {
        name: "Standard_LRS", // Local-redundant storage
    },
    kind: "StorageV2", // General-purpose v2
});

// Export outputs
export const rgName = resourceGroup.name;
export const storageAccountName = storageAccount.name;
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

const redisCache = new azure.cache.Redis("myRedisCache", {
    location: "EastUS",
    resourceGroupName: "myResourceGroup",
    sku: {
        name: "Standard",
        family: "C",
        capacity: 1, // 1 = 1 GB cache size
    },
    enableNonSslPort: false,
    minimumTlsVersion: "1.2",
});

export const redisHostName = redisCache.hostname;
export const redisPrimaryKey = redisCache.primaryKey;
// @ts-ignore
const storageAccount = new azure.storage.StorageAccount("myStorageAccount", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    sku: {
        name: "Standard_LRS",
    },
    kind: "StorageV2",
});

const storageContainer = new azure.storage.BlobContainer("myContainer", {
    resourceGroupName: "myResourceGroup",
    accountName: storageAccount.name,
    publicAccess: "Blob", // Allow public access to blobs
});

export const storageAccountName = storageAccount.name;
export const containerName = storageContainer.name;

const keyVault = new azure.keyvault.Vault("myKeyVault", {
    resourceGroupName: "myResourceGroup",
    location: "EastUS",
    properties: {
        sku: { name: "standard", family: "A" },
        tenantId: "YOUR_TENANT_ID", // Replace with your Azure tenant ID
        accessPolicies: [], // Add access policies here
    },
});

export const keyVaultUri = keyVault.properties?.vaultUri;
